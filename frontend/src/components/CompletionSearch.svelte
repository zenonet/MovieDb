<script lang="ts">
    import { onMount, untrack } from "svelte";


    interface Props{
        placeholder: string,
        items: any[],
        resolveName: (item: any) => string,
        completionClicked: (item:any) => void,
    }

    let params: Props = $props();

    let searchVal = $state("");

    let completions: any[] = $state([])

    function updateCompletions(){
        completions = params.items.filter(item => {
            let name = params.resolveName(item).toLowerCase();
            return name.includes(searchVal.toLowerCase());
        });
    }

    $effect(() => {
        params.items;
        untrack(() => {
            updateCompletions();
        });
    });
    onMount(updateCompletions);
</script>

<div>
    <input bind:value={searchVal} placeholder={params.placeholder} oninput={updateCompletions}>
    <div id="completionsBox">
        {#each completions as completion}
            <button onclick={() => params.completionClicked(completion)}>
                {params.resolveName(completion)}
            </button>
        {/each}
    </div>
</div>

<style>
    #completionsBox{
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 5pt;
        border: var(--foreground) 1px solid;
    }

    #completionsBox button{
        background: none;
        border:none;
        font-weight: unset;
        padding: unset;
    }
</style>