import { getWatchlist } from '$lib/services/watchlistService';

export const load = async ({ params }) => {

    return await getWatchlist(params.id);
};