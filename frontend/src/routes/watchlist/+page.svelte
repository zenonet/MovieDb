<script lang="ts">
    import type { Watchlist } from "$lib/services/types";

    let {data}:{data: {watchlists: Watchlist[]}} = $props();
    let watchlists = data.watchlists;
</script>


<h1>Watchlists</h1>

<div class="list">
    {#each watchlists as watchlist}
        <a href={`/watchlist/${watchlist.id}`}>{watchlist.name}</a>
    {/each}
</div>