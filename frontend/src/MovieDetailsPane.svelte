<script lang="ts">
    import { onMount } from "svelte";
    import { untrack } from 'svelte';

    let {movieId} = $props();


    interface MovieDetails{
        name:string,
        id: string,
        nights: [any]
    }

    let details: MovieDetails|null = $state(null);
    async function fetchMovieDetails(){
        let resp = await fetch(`http://localhost:2222/movie/${movieId}`);
        details = await resp.json();
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


{#if details}
<div>
    <h2>{details.name}</h2>
    <div style="background-color: gray; padding: 15px">
        <h2>Movie nights</h2>
        <div>
            {#each details.nights as night}
                <a href={`/night/${night.id}`}>{new Date(night.time).toLocaleDateString(window.navigator.language)}</a>
            {/each}
            {#if details.nights.length == 0}
                This movie was never presented yet
            {/if}
        </div>
    </div>
</div>
<!-- 
<pre>
    { JSON.stringify(details) }
</pre> -->
{/if}