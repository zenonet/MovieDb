<script>
    import { selectedMovieInList } from "$lib/index.svelte";
    import MovieDetails from "../dataViews/MovieDetails.svelte";
    import MovieList from "../MovieList.svelte";

</script>

<div class="side-by-side-layout">
    <div style="flex: 1">
    <MovieList
        onMovieClicked={(movie) => {
            selectedMovieInList.set(movie.id)
        }}
        pageSize={1000}
    />
    </div>

    {#if selectedMovieInList.selectedId !== ""} 
    <div style="flex: 3; overflow: auto">
        <MovieDetails movieId={selectedMovieInList.selectedId}/>
    </div>
    {/if}
</div>

<style>
    .side-by-side-layout{
        display: flex;
        flex-direction: row;
        max-height: 100%;
        overflow-y: hidden;
        flex: 1;
    }
</style>
