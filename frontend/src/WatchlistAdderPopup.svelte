<script lang="ts">
    import type { Movie, Watchlist } from "$lib/services/types";
    import { addToWatchlist, getWatchlists } from "$lib/services/watchlistService";
    import { onMount } from "svelte";
    import CompletionSearch from "./components/CompletionSearch.svelte";

    let { movie, done }: { movie: Movie, done: () => void } = $props();
    let watchlists: Watchlist[] = $state([]);

    async function completionClicked(comp: Watchlist){
        const idx = await addToWatchlist(comp.id, movie.id);
        console.log(`Inserted ${movie.name} into watchlist ${comp.name} at index ${idx}`);
        done();
    }

    onMount(async () => {
        watchlists = await getWatchlists();
    });
</script>

<div>
    <CompletionSearch
        items={watchlists}
        resolveName={w => w.name}
        placeholder="Search for watchlists"
        completionClicked={completionClicked}
    />
</div>
