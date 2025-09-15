<script lang="ts">
    import { invalidate } from "$app/navigation";
    import { base } from "$app/paths";
    import type { Watchlist } from "$lib/services/types";
    import { createWatchlist } from "$lib/services/watchlistService";

    let {data}:{data: {watchlists: Watchlist[]}} = $props();
    let watchlists = $derived(data.watchlists);

    let newName = $state("");
    async function createButtonClicked(){
        await createWatchlist(newName);
        await invalidate("watchlist:all")
    }
</script>


<h1>Watchlists</h1>

<div>
    <h2>Create new</h2>
    <label for="newName">Name:</label>
    <input id="newName" placeholder="My Watchlist" bind:value={newName}>
    <button onclick={createButtonClicked}>Create</button>
</div>

<div class="list">
    {#each watchlists as watchlist}
        <a href={`${base}/watchlist/${watchlist.id}`}>{watchlist.name}</a>
    {/each}
</div>