import { getWatchlist } from '$lib/services/watchlistService';

export const load = async ({ params, depends }) => {
    depends(`watchlist:${params.id}`);
    return await getWatchlist(params.id);
};