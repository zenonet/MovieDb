import { PUBLIC_API_URL } from "$env/static/public";
import axios from "axios";
import type { Watchlist } from "./types";

const apiClient = axios.create({
    baseURL: PUBLIC_API_URL + "/watchlist"
});

export let createWatchlist = async (name: string) => {
    const resp = await apiClient.post("", {
        name
    });

    let id = resp?.data as string | null;
    return id;
}


export let getWatchlist = async (id: string) => {
    const resp = await apiClient.get(`${id}/`);
    const watchlist = resp.data as Watchlist;
    return watchlist;
};

export let addToWatchlist = async (watchlistId: string, movieId: string) => {
    let resp = await apiClient.post(`${watchlistId}/`, {
        movie: movieId,
    });

    const idx = Number(resp.data as string);
    return idx
}

export let removeFromWatchlist = async (watchlistId: string, idx: number) => {
    apiClient.delete(`${watchlistId}/${idx}`)
}