<script lang="ts">
    import type { Person } from "$lib/services/types";

    let { data }: {data: {persons: Person[]}} = $props();
    let persons = data.persons;

</script>

<h1>People</h1>

<div class="list">
    {#each persons as person}
        <a href={`/person/${person.id}`}>{person.name}</a>
    {/each}
</div>