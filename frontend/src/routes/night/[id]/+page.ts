import { PUBLIC_API_URL } from "$env/static/public";
import { getNightById, getRatingsForNight } from "$lib/services/movieService.js";

export const load = async ({ params }) => {
    const details = await getNightById(params.id);
    const ratings = await getRatingsForNight(params.id);

    return {
        details: details.data,
        ratings: ratings.data,
    }
};