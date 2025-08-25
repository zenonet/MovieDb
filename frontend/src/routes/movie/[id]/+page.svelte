<script>
    import { page } from "$app/state";
    import MovieDetails from "../../../dataViews/MovieDetails.svelte";

    let id = page.params.id;
</script>

<div style="overflow: auto; display: flex; flex-direction: column;">
<MovieDetails movieId={id}/>
</div>