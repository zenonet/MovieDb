<script lang="ts">
    import { onMount } from "svelte";

    interface Props {
        placeholder: string;
        fetchItems: (search: string) => Promise<any[]>;
        resolveName: (item: any) => string;
        completionClicked: (item: any) => void;
        hideCompletionsOnFocusLoss?: boolean;
    }

    let params: Props = $props();

    let searchVal = $state("");

    let completions: any[] = $state([]);

    async function updateCompletions() {
        completions = await params.fetchItems(searchVal);
    }

    onMount(updateCompletions);

    let showCompletions = $state(!params.hideCompletionsOnFocusLoss);
</script>

<div
    onfocusin={() => showCompletions = true}
    onfocusout={() => showCompletions = !params.hideCompletionsOnFocusLoss}
>
    <input
        bind:value={searchVal}
        placeholder={params.placeholder}
        oninput={updateCompletions}
    />
    <div id="completionsBox" style:display={(showCompletions ? "flex" : "none")}>
        {#each completions as completion}
            <button onmousedown={(e) => {params.completionClicked(completion); e.preventDefault()}}>
                {params.resolveName(completion)}
            </button>
        {/each}
    </div>
</div>

<style>
    #completionsBox {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    #completionsBox button {
        background: none;
        border: none;
    }
</style>
