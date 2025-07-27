<script lang="ts">
    let { data } = $props();
    let person = $derived(data.details);
</script>

{#if person}
    <h1>{person.name}</h1>

    <h3>Latest films watched</h3>

    {#each person.latest_nights as night}
        <a href={`/night/${night.id}`}>{night.movie.name} on {new Date(night.time).toLocaleDateString(navigator.language)}</a>
    {/each}
{/if}