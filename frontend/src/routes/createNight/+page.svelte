<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
    import { getMovieById, getPersonsByName, postNight, postRating } from "$lib/services/movieService";
    import type { Movie, Person } from "$lib/services/types";
    import { onMount } from "svelte";

    let movieId: string = page.url.searchParams.get("movieId")!!;
    let movie: Movie | null = $state(null);

    onMount(() =>
        getMovieById(movieId).then((m) => {
            movie = m.data;
        }),
    );

    let description = $state("");

    interface PersonWithRating extends Person{
        rating?: number
    }

    var now = new Date();
    now.setMinutes(now.getMinutes() - now.getTimezoneOffset());

    let date = $state(now.toISOString().slice(0, 16));

    let searchVal = $state("");
    let completions: Array<Person> = $state([]);
    let people: Array<PersonWithRating> = $state([]);
    let showCompletionWindow = $state(false);

    async function searchInputChanged() {
        let res = await getPersonsByName(searchVal);
        if (res.status != 200) return;

        completions = res.data.filter(
            (c) => !people.map((p) => p.id).includes(c.id),
        );
    }

    function completionClicked(person: Person) {
        people.push(person as PersonWithRating);

        searchVal = "";
        searchInputChanged();
    }

    function removePerson(person: Person) {
        // These are actually the same objects so we can compare by reference instead of by id like in searchInputChanged
        people = people.filter((p) => p !== person);
    }

    async function create() {
        
        // Post the night itself
        let d = description == "" ? null : description;
        let viewIds = await postNight(movie!!, d, people.map(p => p.id));

        // Now, post the ratings
        for(let p of people){
            let viewId = viewIds.get(p.id)!!;

            await postRating(viewId, p.rating!!);
            goto("/");
        }
        console.log("Night created successfully!")
    }
</script>

<div
    style="display: flex; flex-direction:column; align-items: flex-start; padding: 10pt"
>
    <h1>Create new night</h1>

    <label for="date">Movie:</label>
    <select disabled>
        <option>{movie?.name || "Loading..."}</option>
    </select>

    <label for="date">Time:</label>
    <input type="datetime-local" id="date" bind:value={date} />

    <label for="description-field">Description (optional):</label>
    <textarea
        id="description-field"
        bind:value={description}
        placeholder="Write something about this movie-night..."
    ></textarea>

    <h3>Who watched with you?</h3>

    <div>
        <div>
            <input
                id="searchField"
                onfocus={() => {
                    showCompletionWindow = true;
                    searchInputChanged();
                }}
                bind:value={searchVal}
                style="width: 100%"
                oninput={searchInputChanged}
            />
            <div
                id="completionWindow"
                style="width: 100%;"
                style:max-height={showCompletionWindow ? "20cm" : "0"}
            >
                {#each completions as completion}
                    <button onclick={() => completionClicked(completion)}
                        >{completion.name}</button
                    >
                {/each}
            </div>
        </div>
        <table style="margin-top: 40px">
            <thead>
                <tr>
                    <td>Name</td>
                    <td>Rating</td>
                </tr>
            </thead>
            <tbody>
                {#each people as person}
                    <tr>
                        <td>{person.name}</td>
                        <td>
                            <input type="number"min="0" max="10" value="0" onchange={(el) => person.rating = Number((el.target as HTMLInputElement).value)}>
                        </td>
                        <td>
                            <button
                                class="remove-button"
                                onclick={() => removePerson(person)}>X</button
                            >
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>

    <button onclick={create} style="margin-top: 40px">Create</button>
</div>

<style>
    /* Create some distance between points which start with a label*/
    label::before {
        display: block;
        height: 15px;
        content: "";
    }

    #completionWindow {
        border: black 1px solid;
        overflow: clip;
        border-top: none;
        border-bottom-left-radius: 10px;
        border-bottom-right-radius: 10px;

        transition-property: max-height height;
        transition-duration: 500ms;

        display: flex;
        flex-direction: column;
        gap: 5px;
        align-items: flex-start;
    }
    #completionWindow button {
        background-color: var(--background);
        border: none;
        display: flex;
    }

    .remove-button {
        background: var(--background);
        border: none;
    }

    table{
        table-layout: fixed;
    }
</style>
