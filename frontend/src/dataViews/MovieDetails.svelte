<script lang="ts">
    import { goto } from "$app/navigation";
    import { PUBLIC_API_URL } from "$env/static/public";
    import type { MovieDetails } from "$lib/services/types";
    import { onMount } from "svelte";
    import { untrack } from 'svelte';

    let {movieId} = $props();

    let movie: MovieDetails|null = $state(null);
    async function fetchMovieDetails(){
        let resp = await fetch(`${PUBLIC_API_URL}/movie/${movieId}`);
        movie = await resp.json();
        console.log("Movie fetched")
    }


    $effect(() => {
		if(!movieId) return;
		untrack(() => {
			fetchMovieDetails();
		});
	});
    onMount(()=>{
        fetchMovieDetails();
    });


</script>


{#if movie}
<div style="display: flex; flex-direction: column; align-items: flex-start">
    <h2>{movie.name}</h2>

    <button onclick={() => goto(`/createNight?movieId=${movie!!.id}`)}>Create night</button>
    
    {#if movie.coverUrl}
        <img src={movie.coverUrl} alt={`Movie cover of ${movie.name}`} height="400px">
    {/if}

    {#if movie.avgRating}
        Average Rating: {movie.avgRating!!.toFixed(2)}
    {/if}
    <h3>Movie nights</h3>
    
    <div class="list">
        {#each movie.nights as night}
            <a href={`/night/${night.id}`}>{new Date(night.time).toLocaleDateString(window.navigator.language)} (rated: {night.avgRating})</a>
        {/each}
        {#if movie.nights.length == 0}
            This movie was never presented yet
        {/if}
    </div>
</div>
{/if}