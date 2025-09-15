<script lang="ts">
    import { goto } from "$app/navigation";
    import { getMovieById } from "$lib/services/movieService";
    import type { MovieDetails } from "$lib/services/types";
    import { onMount, untrack } from "svelte";
    import WatchlistAdderPopup from "../WatchlistAdderPopup.svelte";

    let { movieId } = $props();

    let movie: MovieDetails | null = $state(null);
    async function fetchMovieDetails() {
        movie = (await getMovieById(movieId)).data;
    }

    $effect(() => {
        if (!movieId) return;
        untrack(() => {
            fetchMovieDetails();
        });
    });
    onMount(() => {
        fetchMovieDetails();
    });

    let showWatchlistAdder = $state(false);

    function getActorList(actors: string) {
        return actors.split("\n").map((line) => {
            const idx = line.lastIndexOf(" als ");
            const actor = line.substring(0, idx);
            const role = line.substring(idx + 5);
            return {
                role,
                actor,
            };
        });
    }
</script>

{#if movie}
    <div
        style="display: flex; flex-direction: column; align-items: flex-start; flex: 1; overflow: auto"
    >
        {#if movie.coverUrl}
            <img
                src={movie.coverUrl}
                alt={`Movie cover of ${movie.name}`}
                height="400px"
            />
        {/if}
        <h2>{movie.name}</h2>

        <div style="display: flex; gap: 10pt">
            <button onclick={() => goto(`/createNight?movieId=${movie!!.id}`)}
                >Create night</button
            >
            <button onclick={() => (showWatchlistAdder = true)}
                >Add to a watchlist</button
            >
        </div>

        {#if showWatchlistAdder}
            <WatchlistAdderPopup
                {movie}
                done={() => (showWatchlistAdder = false)}
            ></WatchlistAdderPopup>
        {/if}

        {#if movie.yearOfPublication}
            {movie.yearOfPublication}
        {/if}

        {#if movie.duration}
            Duration: {movie.duration}min
        {/if}

        {#if movie.description}
            <p style="font-size: 0.8em">{movie.description}</p>
        {/if}

        {#if movie.avgRating}
            Average Rating: {movie.avgRating!!.toFixed(2)}
        {/if}

        <h3>Movie nights</h3>

        <div class="list">
            {#each movie.nights as night}
                <a href={`/night/${night.id}`}
                    >{new Date(night.time).toLocaleDateString(
                        window.navigator.language,
                    )} (rated: {night.avgRating})</a
                >
            {/each}
            {#if movie.nights.length == 0}
                This movie was never presented yet
            {/if}
        </div>

        <h3>More Details</h3>
        {#if movie.trailerUrl}
            <a href={movie.trailerUrl}>Trailer</a>
        {/if}

        {#if movie.tagline}
            <span style="font-size: 1.2em; font-style: italic;"
                >{movie.tagline}</span
            >
        {/if}

        {#if movie.isMementoImport}
            <span style="font-weight: light;"
                >This movie was imported from a memento database</span
            >
        {/if}

        {#if movie.actors}
            <h2>Cast</h2>
            <table>
                <thead style="font-weight: bold">
                    <tr>
                        <td>Actor</td>
                        <td>Character name</td>
                    </tr>
                </thead>
                <tbody>
                    {#each getActorList(movie.actors) as l}
                        <tr>
                            <td>
                                <a href={`/actor/${encodeURI(l.actor)}`}
                                    >{l.actor}</a
                                >
                            </td>
                            <td>{l.role}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
    </div>
{/if}
