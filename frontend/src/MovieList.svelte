<script lang="ts">
    import { PUBLIC_API_URL } from "$env/static/public";
    import { getMoviesByName } from "$lib/services/movieService";
    import type { Movie } from "$lib/services/types";

    let { onMovieClicked, pageSize } = $props<{ onMovieClicked: (id:Movie) => void, pageSize: number}>();

    let searchVal = $state("");

    let movies: Movie[] = $state([]);

    async function fetchMovies(){
        let res = await getMoviesByName(searchVal, 0, pageSize);
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