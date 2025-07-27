<script>
    import { page } from "$app/state";
    import MovieDetails from "../../../dataViews/MovieDetails.svelte";

    let id = page.params.id;
</script>

<MovieDetails movieId={id}/>