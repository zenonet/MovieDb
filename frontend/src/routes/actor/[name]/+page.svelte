<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";

    let { data } = $props();
    let movies = $derived(data.movies);
</script>

<h1>{page.params.name}</h1>
<h3>Movies in DB:</h3>
<div class="movie-list">
    {#each movies as movie}
        <button
            class="movie-list-entry"
            onclick={() => goto(`/movie/${movie.id}`)}
        >
            {movie.name}
        </button>
    {/each}
</div>
