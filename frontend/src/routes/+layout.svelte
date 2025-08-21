<script lang="ts">
    let { children } = $props();
</script>

<div class="root">
    <nav>
        <a href="/">Movies</a>
        <a href="/watchlist">Watchlists</a>
        <a href="/person">People</a>
    </nav>

    <div style="flex: 1; overflow-y: hidden; padding: 15px">
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
