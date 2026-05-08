<script>
  import '../app.css';
</script>

<svelte:head>
  <title>CompOS Installer</title>
  <meta name="description" content="Modern Linux installation with beautiful UI" />
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-compos-primary to-compos-secondary">
  <slot />
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
      'Ubuntu', 'Cantarell', 'Open Sans', 'Helvetica Neue', sans-serif;
  }
</style>
