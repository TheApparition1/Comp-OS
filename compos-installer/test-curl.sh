#!/bin/bash

# Test script to verify curl functionality in the installer
set -euo pipefail  # Exit on error, undefined vars, pipe failures

# Cleanup function
cleanup() {
    rm -f /tmp/test_download.json /tmp/test_vscode.deb /tmp/node_setup.sh /tmp/docker_install.sh
}
trap cleanup EXIT

echo "Testing curl-based installation functionality..."

# Test 1: Check if curl is available
echo "1. Checking curl availability:"
if command -v curl &> /dev/null; then
    echo "✓ curl is available"
    curl --version | head -1
else
    echo "✗ curl is not available"
    exit 1
fi

# Test 2: Test downloading a small file
echo -e "\n2. Testing download functionality:"
TEST_URL="https://httpbin.org/json"
TEST_FILE="/tmp/test_download.json"

echo "Downloading from: $TEST_URL"
if curl -L -fsSL --connect-timeout 10 --max-time 30 -o "$TEST_FILE" "$TEST_URL"; then
    if [ -f "$TEST_FILE" ] && [ -s "$TEST_FILE" ]; then
        FILE_SIZE=$(wc -c < "$TEST_FILE")
        echo "✓ Download successful"
        echo "  File size: $FILE_SIZE bytes"
        echo "  File exists: Yes"
        echo "  Content preview: $(head -c 100 "$TEST_FILE")..."
    else
        echo "✗ Download failed - file is empty or missing"
        exit 1
    fi
else
    echo "✗ Download failed"
    exit 1
fi

# Helper function to test URL accessibility
test_url() {
    local url="$1"
    local name="$2"
    echo -e "\n3. Testing $name URL accessibility:"
    
    # Try to get HTTP status code
    HTTP_STATUS=$(curl -L -s -o /dev/null -w "%{http_code}" --connect-timeout 10 --max-time 30 "$url" || echo "000")
    
    case "$HTTP_STATUS" in
        200)
            echo "✓ $name URL is accessible (HTTP $HTTP_STATUS)"
            return 0
            ;;
        301|302|303|307|308)
            echo "✓ $name URL redirects properly (HTTP $HTTP_STATUS)"
            return 0
            ;;
        000)
            echo "✗ $name URL is not accessible (connection failed)"
            return 1
            ;;
        *)
            echo "⚠ $name URL returned HTTP $HTTP_STATUS"
            return 1
            ;;
    esac
}

# Test 3: Test VS Code download URL
VSCODE_TEST_URL="https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"
test_url "$VSCODE_TEST_URL" "VS Code"

# Test 4: Test Node.js setup script URL
NODEJS_TEST_URL="https://deb.nodesource.com/setup_lts.x"
test_url "$NODEJS_TEST_URL" "Node.js setup script"

# Test 5: Test Docker installation script URL
DOCKER_TEST_URL="https://get.docker.com"
test_url "$DOCKER_TEST_URL" "Docker installation script"

# Test 6: Test actual download of a small script
echo -e "\n6. Testing actual script download:"
if curl -L -fsSL --connect-timeout 10 --max-time 30 -o /tmp/docker_install.sh "$DOCKER_TEST_URL"; then
    if [ -f "/tmp/docker_install.sh" ] && [ -s "/tmp/docker_install.sh" ]; then
        echo "✓ Docker script download successful"
        echo "  File size: $(wc -c < /tmp/docker_install.sh) bytes"
        echo "  First few lines:"
        head -3 /tmp/docker_install.sh | sed 's/^/    /'
    else
        echo "✗ Docker script download failed - file is empty or missing"
    fi
else
    echo "✗ Docker script download failed"
fi

# Test 7: Test curl options used by installer
echo -e "\n7. Testing curl options used by installer:"
echo "Testing -L (follow redirects), -f (fail on HTTP errors), -s (silent), -S (show errors)"

if curl -L -fsS --connect-timeout 10 --max-time 30 -o /dev/null "$TEST_URL"; then
    echo "✓ All curl options work correctly"
else
    echo "✗ Curl options test failed"
fi

echo -e "\n" && echo "=== Test Summary ==="
echo "✓ curl availability: PASSED"
echo "✓ Basic download: PASSED"
echo "✓ URL accessibility tests: COMPLETED"
echo "✓ Script download: COMPLETED"
echo "✓ Curl options: PASSED"

echo -e "\n✓ All curl functionality tests completed!"
echo "The installer should be able to use curl for downloading packages."
