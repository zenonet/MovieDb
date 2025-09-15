<script lang="ts">
    import "$lib/app.css"
    let { children } = $props();
</script>

<div class="root">
    <nav>
        <a href="/">Movies</a>
        <a href="/watchlist">Watchlists</a>
        <a href="/person">People</a>
    </nav>

    <div style="flex: 1; padding: 15px; flex-shrink: 1; min-height: 0">
        {@render children()}
    </div>
</div>

<style>
    nav {
        padding: 15px;
        display: flex;
        gap: 10px;
    }

    .root{
        height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>
