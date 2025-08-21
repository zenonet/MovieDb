<script lang="ts">
    import { PUBLIC_API_URL } from "$env/static/public";
    import { getMoviesByName } from "$lib/services/movieService";
    import type { Movie } from "$lib/services/types";

    let { onMovieClicked } = $props<{ onMovieClicked: (id:Movie) => void}>();

    let searchVal = $state("");

    let movies: Movie[] = $state([]);

    async function fetchMovies(){
        let res = await getMoviesByName(searchVal);
        if (res.status == 200){
            movies = res.data;
        }
    }

    fetchMovies();

</script>

<div style="display: flex; flex-direction:column;">
    <input placeholder="Search..." bind:value={searchVal} oninput={fetchMovies}>

    <div class="movie-list">
        {#each movies as movie}
            <button class="movie-list-entry" onclick={() => onMovieClicked(movie)}>
                { movie.name }
            </button>
        {/each}
    </div>
</div>

<style>
    .movie-list-entry{
        background: var(--background);

        border: none;
		text-align: left;
        font-size: 1.2em;
    }

    .movie-list{
        margin-top: 10px;
        display: flex;
        flex-direction: column;
        overflow: auto;
        flex: 1;

        max-height: 100%;

        gap: 10px;
    }
</style>