import { PUBLIC_API_URL } from "$env/static/public";
import { getNightById } from "$lib/services/movieService.js";

export const load = async ({ params }) => {
    return await getNightById(params.id)
};