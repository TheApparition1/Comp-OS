<script>
  import '../app.css';
</script>

<svelte:head>
  <title>CompOS Installer</title>
  <meta name="description" content="Modern Linux installation with beautiful UI" />
</svelte:head>

<slot />

<style>
  :global(body) {
    margin: 0;
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  }
</style>
