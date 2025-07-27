<script>
    import MovieList from "../MovieList.svelte";

</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
<MovieList onMovieClicked={(movie) => console.log(`${movie.name} clicked`)}/>