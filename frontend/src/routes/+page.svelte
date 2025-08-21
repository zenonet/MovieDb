<script>
    import MovieDetails from "../dataViews/MovieDetails.svelte";
    import MovieList from "../MovieList.svelte";


    let selectedId = $state("");
</script>

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
        height: 100%;
    }
</style>
