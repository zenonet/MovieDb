<script lang="ts">
    let {children} = $props();
</script>

<nav>
    <a href="/">Movies</a>
    <a href="/watchlist">Watchlists</a>
</nav>

{@render children()}


<style>
    nav{
        padding: 15px;
        display: flex;
        gap: 10px;
    }
</style>