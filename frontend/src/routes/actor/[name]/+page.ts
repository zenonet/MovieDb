import { getMoviesWithActor } from "$lib/services/movieService";

export const load = async ({params}) => {

    const movies = (await getMoviesWithActor(params.name)).data;
    return {
        movies
    }
};