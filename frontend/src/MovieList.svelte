<script lang="ts">

    class Movie{
        name!: string;
        id!: string;
    }

    let { onMovieClicked } = $props<{ onMovieClicked: (id:Movie) => void}>();

    let searchVal = $state("");

    let movies: Movie[] = $state([]);

    async function fetchMovies(){
        console.log(searchVal)
        const resp = await fetch(`http://localhost:2222/movie?page=0&per_page=100&name=${encodeURI(searchVal)}`);
        movies = await resp.json();
    }

    fetchMovies();

</script>

<div>
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

        gap: 10px;
    }
</style>