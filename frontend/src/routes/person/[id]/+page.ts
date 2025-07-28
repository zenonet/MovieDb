import { PUBLIC_API_URL } from '$env/static/public';
import { getPersonById } from '$lib/services/movieService';

export const load = async ({ params }) => {

    return await getPersonById(params.id);
};