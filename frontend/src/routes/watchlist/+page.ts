import { getWatchlists } from '$lib/services/watchlistService';

export const load = async ({ params }) => {

    const watchlists = await getWatchlists();
    return {
        watchlists
    };
};