<script lang="ts">
    import { PUBLIC_API_URL } from "$env/static/public";
    import { onMount } from "svelte";
    import { untrack } from 'svelte';

    let {movieId} = $props();


    interface MovieDetails{
        name:string,
        id: string,
        nights: Array<any>
        tagline?: string
        coverUrl?: string
        description?: string
        yearOfPublication?: number
    }

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
<div>
    <h2>{movie.name}</h2>
    
    {#if movie.coverUrl}
        <img src={movie.coverUrl} alt={`Movie cover of ${movie.name}`} height="400px">
    {/if}
    <h3>Movie nights</h3>
    
    <div>
        {#each movie.nights as night}
            <a href={`/night/${night.id}`}>{new Date(night.time).toLocaleDateString(window.navigator.language)}</a>
        {/each}
        {#if movie.nights.length == 0}
            This movie was never presented yet
        {/if}
    </div>
</div>
{/if}