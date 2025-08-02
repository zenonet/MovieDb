import { getPersonById } from '$lib/services/movieService';

export const load = async ({ params }) => {

    return await getPersonById(params.id);
};