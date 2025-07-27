<script>
    import MovieDetails from "../dataViews/MovieDetails.svelte";
    import MovieList from "../MovieList.svelte";


    let selectedId = $state("");
</script>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the
    documentation
</p>
<div class="side-by-side-layout">
    <MovieList
        onMovieClicked={(movie) => selectedId = movie.id}
    />

    {#if selectedId !== ""} 
        <MovieDetails movieId={selectedId}/>
    {/if}
</div>

<style>
    .side-by-side-layout{
        display: flex;
        flex-direction: row;
    }
</style>
