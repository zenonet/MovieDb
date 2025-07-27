<script lang="ts">
    import { onMount } from "svelte";
    import { untrack } from 'svelte';

    let {movieId} = $props();


    let details = $state(null);
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

<pre>
    { JSON.stringify(details) }
</pre>