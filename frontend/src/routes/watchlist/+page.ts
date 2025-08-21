import { getWatchlists } from '$lib/services/watchlistService';

export const load = async ({ depends }) => {

    const watchlists = await getWatchlists();
    depends("watchlist:all");
    return {
        watchlists
    };
};