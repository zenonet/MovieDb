<script lang="ts">
    import type { Watchlist } from "$lib/services/types";

    let { data }: { data: Watchlist } = $props();
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
                </tr>
            {/each}
        </tbody>
    </table>
</div>
