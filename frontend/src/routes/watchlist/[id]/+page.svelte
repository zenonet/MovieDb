<script lang="ts">
    import { invalidate } from "$app/navigation";
import type { WatchlistDetails } from "$lib/services/types";
    import { removeFromWatchlist } from "$lib/services/watchlistService";

    let { data }: { data: WatchlistDetails } = $props();
    let watchlist = data;
</script>

<div style="display: flex; flex-direction: column; align-items: flex-start;">
    <h1>{watchlist.name}</h1>

    {#if watchlist.description}
        <p>{watchlist.description}</p>
    {/if}

    <h2>Movies:</h2>

    <table>
        <thead>
            <tr>
                <td></td>
                <td>Name</td>
                <td></td>
            </tr>
        </thead>
        <tbody>
            {#each data.entries as entry}
                <tr>
                    <td>{entry.idx}</td>
                    <td>
                        <a href={`/movie/${entry.movie.id}`}>
                            {entry.movie.name}
                        </a>
                    </td>
                    <td>
                        <button onclick={async () => {
                                await removeFromWatchlist(watchlist.id, entry.idx);
                                await invalidate(`watchlist:${watchlist.id}`);
                            } }>X</button>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>
