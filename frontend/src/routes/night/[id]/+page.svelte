<script lang="ts">
    import { base } from "$app/paths";
    import type { NightDetails, Rating } from "$lib/services/types";

    let {data}: {data:{details: NightDetails, ratings: Rating[]}} = $props();
    let night = $derived(data.details);
    let ratings = $derived(data.ratings);
</script>

<h1>Night of {new Date(night.time).toLocaleString(navigator.language)}</h1>

{#if night.description}
<p>{night.description}</p>
{/if}


<h3>Which movie?</h3>

<a href={`${base}/movie/${night.movie.id}`}>{night.movie.name}</a>

<h3>Who was watching?</h3>

<div class="list">
{#each night.persons as person}
    <a href={`${base}/person/${person.id}`}>{person.name} (rated: {person.avgRating}{(person.ratingCount > 1 ? `, based on ${person.ratingCount} ratings` : "")})</a>
{/each}
</div>

{#if night.persons.length == 1}
<p>Pretty lonely, huh? I am here for you :)</p>
{/if}

<h3>Ratings</h3>

<table>
    <thead>
        <tr>
            <td>Person</td>
            <td>Rating</td>
            <td>Date</td>
        </tr>
    </thead>
    <tbody>
        {#each ratings as rating}
            <tr>
                <td>
                    <a href={`${base}/person/${rating.person.id}`}>{rating.person.name}</a>
                </td>
                <td>{rating.value.toFixed()}</td>
                <td>{rating.time.toLocaleString(navigator.language)}</td>
            </tr>
        {/each}
    </tbody>
</table>