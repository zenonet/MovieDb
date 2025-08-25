<script lang="ts">
    import { goto } from "$app/navigation";

    let { data } = $props();
    let movies = $derived(data.movies);
</script>

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
