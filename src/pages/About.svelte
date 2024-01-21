<script lang="ts">
    const version = __APP_VERSION__;
    const buildTime = __BUILD_TIME__;
</script>

<main class="container">
    <h1>PassmanTRS</h1>
    <p>Version {version}</p>
    <p>Built on {buildTime}</p>
    <p>PassmanTRS is a password manager written in Rust(Tauri), TypeScript and Svelte.</p>
    <p>&copy; 2023-2024 gamersi (Simon R.) &amp; contributors</p>
</main>

<style>
    .container {
        overflow: hidden;
    }
</style>