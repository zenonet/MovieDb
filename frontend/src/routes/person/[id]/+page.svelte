<script lang="ts">
    import { base } from '$app/paths';

    let { data } = $props();
    let person = $derived(data.data);
</script>

{#if person}
    <h1>{person.name}</h1>

    <h3>Latest films watched</h3>

    <div class="list">
        {#each person.latest_nights as night}
            <a href={`${base}/night/${night.id}`}
                >{night.movie!!.name} on {new Date(
                    night.time,
                ).toLocaleDateString(navigator.language)}</a
            >
        {/each}
    </div>
{/if}
