"use client";

import { useState, useEffect } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { Search, Download, Package, Terminal, Loader2, AlertCircle } from "lucide-react";

interface App {
  id: string;
  name: string;
  description: string;
  category: string;
  installed: boolean;
  version?: string;
  repository?: string;
  source?: string;
}

interface ApiResponse {
  pacman: App[];
  aur: App[];
  mock?: boolean;
  error?: string;
}

export default function Home() {
  const [searchQuery, setSearchQuery] = useState("");
  const [apps, setApps] = useState<App[]>([]);
  const [filteredApps, setFilteredApps] = useState<App[]>([]);
  const [isInstalling, setIsInstalling] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [terminalCommand, setTerminalCommand] = useState<string>("");
  const [terminalOutput, setTerminalOutput] = useState<string>("");

  // Search packages from Pacman and AUR
  const searchPackages = async (query: string) => {
    if (!query || query.trim() === '') {
      setApps([]);
      setFilteredApps([]);
      setError(null);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch(`/api/packages?search=${encodeURIComponent(query.trim())}`);
      console.log('Response status:', response.status);
      console.log('Response ok:', response.ok);
      
      if (!response.ok) {
        const errorText = await response.text();
        console.error('Response error:', errorText);
        throw new Error(`Failed to search packages: ${response.status} ${errorText}`);
      }

      const data: ApiResponse = await response.json();
      console.log('API Response:', data);
      const allPackages = [
        ...data.pacman.map(pkg => ({
          ...pkg,
          id: `pacman-${pkg.name}`,
          category: pkg.repository || 'System',
        })),
        ...data.aur.map(pkg => ({
          ...pkg,
          id: `aur-${pkg.name}`,
          category: 'AUR',
        }))
      ];

      setApps(allPackages);
      setFilteredApps(allPackages);
      
      // Show warning if using mock data
      if (data.mock) {
        setError(data.error || 'Using mock data - Pacman/AUR not available on this system');
      } else {
        setError(null);
      }
    } catch (err) {
      setError('Failed to search packages. Make sure you have Pacman installed.');
      console.error('Search error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // Handle install/uninstall
  const handlePackageAction = async (packageName: string, source: string, action: 'install' | 'uninstall') => {
    setIsInstalling(`${packageName}-${action}`);
    
    try {
      const response = await fetch('/api/packages', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          packageName,
          source,
          action
        }),
      });

      const result = await response.json();
      
      if (result.success) {
        // Refresh the search to update installation status
        await searchPackages(searchQuery);
      } else {
        setError(result.error || 'Failed to execute command');
      }
    } catch (err) {
      setError('Failed to execute package command');
      console.error('Action error:', err);
    } finally {
      setIsInstalling(null);
    }
  };

  // Open terminal with command
  const openTerminal = (packageName: string, source: string, action: 'install' | 'uninstall') => {
    const command = source === 'aur' 
      ? `yay -S ${packageName}`
      : `sudo pacman -S ${packageName}`;
    
    setTerminalCommand(command);
    setTerminalOutput(`Terminal command ready. You can run this command in your terminal:\n\n${command}\n\nNote: You may need sudo privileges for Pacman operations.`);
  };

  // Debounced search
  useEffect(() => {
    const timer = setTimeout(() => {
      searchPackages(searchQuery);
    }, 500);

    return () => clearTimeout(timer);
  }, [searchQuery]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800">
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <div className="text-center mb-8">
          <div className="flex items-center justify-center gap-2 mb-4">
            <Package className="h-8 w-8 text-blue-600" />
            <h1 className="text-4xl font-bold text-slate-900 dark:text-slate-50">
              Arch Package Installer
            </h1>
          </div>
          <p className="text-lg text-slate-600 dark:text-slate-400 max-w-2xl mx-auto">
            Search and install packages from Pacman repositories and AUR
          </p>
        </div>

        {/* Search Bar */}
        <div className="max-w-2xl mx-auto mb-8">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-slate-400" />
            <Input
              type="text"
              placeholder="Search Pacman and AUR packages..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10 h-12 text-base"
            />
            {isLoading && (
              <Loader2 className="absolute right-3 top-1/2 transform -translate-y-1/2 h-4 w-4 animate-spin text-slate-400" />
            )}
          </div>
        </div>

        {/* Error Message */}
        {error && (
          <div className="max-w-2xl mx-auto mb-6">
            <div className={`flex items-center gap-2 p-4 rounded-lg border ${
              error.includes('mock') || error.includes('mock data') 
                ? 'bg-yellow-50 border-yellow-200 dark:bg-yellow-900/20 dark:border-yellow-800' 
                : 'bg-red-50 border-red-200 dark:bg-red-900/20 dark:border-red-800'
            }`}>
              <AlertCircle className={`h-5 w-5 ${
                error.includes('mock') || error.includes('mock data')
                  ? 'text-yellow-600 dark:text-yellow-400'
                  : 'text-red-600 dark:text-red-400'
              }`} />
              <p className={`text-sm ${
                error.includes('mock') || error.includes('mock data')
                  ? 'text-yellow-700 dark:text-yellow-300'
                  : 'text-red-700 dark:text-red-300'
              }`}>{error}</p>
            </div>
          </div>
        )}

        {/* App Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {filteredApps.map((app) => (
            <Card key={app.id} className="hover:shadow-lg transition-shadow duration-200">
              <CardHeader className="pb-3">
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <CardTitle className="text-lg font-semibold text-slate-900 dark:text-slate-50">
                      {app.name}
                    </CardTitle>
                    <div className="flex gap-2 mt-2">
                      <Badge variant="secondary" className="text-xs">
                        {app.category}
                      </Badge>
                      {app.source === 'aur' && (
                        <Badge variant="outline" className="text-xs text-purple-600 border-purple-600">
                          AUR
                        </Badge>
                      )}
                      {app.installed && (
                        <Badge variant="default" className="bg-green-500 hover:bg-green-600">
                          Installed
                        </Badge>
                      )}
                    </div>
                    {app.version && (
                      <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                        v{app.version}
                      </p>
                    )}
                  </div>
                </div>
              </CardHeader>
              <CardContent className="pt-0">
                <CardDescription className="text-sm text-slate-600 dark:text-slate-400 mb-4 line-clamp-3">
                  {app.description}
                </CardDescription>
                <div className="flex gap-2">
                  <Button
                    onClick={() => handlePackageAction(app.name, app.source || 'pacman', app.installed ? 'uninstall' : 'install')}
                    disabled={isInstalling === `${app.name}-${app.installed ? 'uninstall' : 'install'}`}
                    className="flex-1"
                    variant={app.installed ? "destructive" : "default"}
                    size="sm"
                  >
                    {isInstalling === `${app.name}-${app.installed ? 'uninstall' : 'install'}` ? (
                      <>
                        <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                        {app.installed ? "Removing..." : "Installing..."}
                      </>
                    ) : (
                      <>
                        <Download className="h-4 w-4 mr-2" />
                        {app.installed ? "Remove" : "Install"}
                      </>
                    )}
                  </Button>
                  
                  <Dialog>
                    <DialogTrigger asChild>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => openTerminal(app.name, app.source || 'pacman', app.installed ? 'uninstall' : 'install')}
                      >
                        <Terminal className="h-4 w-4" />
                      </Button>
                    </DialogTrigger>
                    <DialogContent className="max-w-2xl">
                      <DialogHeader>
                        <DialogTitle>Terminal Command</DialogTitle>
                        <DialogDescription>
                          Copy and run this command in your terminal
                        </DialogDescription>
                      </DialogHeader>
                      <div className="space-y-4">
                        <div className="p-4 bg-slate-100 dark:bg-slate-800 rounded-lg font-mono text-sm">
                          <code>{terminalCommand}</code>
                        </div>
                        <div className="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg text-sm">
                          <p className="text-blue-700 dark:text-blue-300">
                            {terminalOutput}
                          </p>
                        </div>
                        <Button
                          onClick={() => {
                            navigator.clipboard.writeText(terminalCommand);
                            setTerminalOutput(prev => prev + '\n\nCommand copied to clipboard!');
                          }}
                          variant="outline"
                          className="w-full"
                        >
                          Copy Command
                        </Button>
                      </div>
                    </DialogContent>
                  </Dialog>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>

        {/* No Results */}
        {!isLoading && searchQuery && filteredApps.length === 0 && (
          <div className="text-center py-12">
            <Package className="h-16 w-16 text-slate-300 mx-auto mb-4" />
            <h3 className="text-xl font-semibold text-slate-900 dark:text-slate-50 mb-2">
              No packages found
            </h3>
            <p className="text-slate-600 dark:text-slate-400">
              Try adjusting your search terms or check if the package exists in Pacman or AUR
            </p>
          </div>
        )}

        {/* Initial State */}
        {!searchQuery && (
          <div className="text-center py-12">
            <Search className="h-16 w-16 text-slate-300 mx-auto mb-4" />
            <h3 className="text-xl font-semibold text-slate-900 dark:text-slate-50 mb-2">
              Search for packages
            </h3>
            <p className="text-slate-600 dark:text-slate-400">
              Enter a package name to search Pacman repositories and AUR
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
