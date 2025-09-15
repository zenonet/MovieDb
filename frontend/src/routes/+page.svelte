<script>
    import { selectedMovieInList } from "$lib/index.svelte";
    import MovieDetails from "../dataViews/MovieDetails.svelte";
    import MovieList from "../MovieList.svelte";

</script>

<div class="side-by-side-layout">
    <MovieList
        onMovieClicked={(movie) => {
            selectedMovieInList.set(movie.id)
        }}
        pageSize={1000}
    />

    {#if selectedMovieInList.selectedId !== ""} 
        <MovieDetails movieId={selectedMovieInList.selectedId}/>
    {/if}
</div>

<style>
    .side-by-side-layout{
        display: flex;
        flex-direction: row;
        height: 100%;
    }
</style>
