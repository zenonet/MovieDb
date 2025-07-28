<script lang="ts">
    let {data} = $props();
    let night = $derived(data.data);
</script>

<h1>Night of {new Date(night.time).toLocaleString(navigator.language)}</h1>

{#if night.description}
<p>{night.description}</p>
{/if}


<h3>Which movie?</h3>

<a href={`/movie/${night.movie.id}`}>{night.movie.name}</a>

<h3>Who was watching?</h3>

{#each night.persons as person}
    <a href={`/person/${person.id}`}>{person.name}</a>
{/each}

{#if night.persons.length == 1}
<p>Pretty lonely, huh? I am here for you :)</p>
{/if}

<pre>
    {JSON.stringify(night)}
</pre>