import { PUBLIC_API_URL } from "$env/static/public";
import axios from "axios";
import type { Movie, MovieDetails, NightDetails, PersonDetails } from "./types";

const apiClient = axios.create({
    baseURL: PUBLIC_API_URL
})

export let getMovieById = (id: string) => {
    return apiClient.get<MovieDetails>(`/movie/${id}`)
};

export let getMoviesByName = (name: string, page:number = 0) => {
    return apiClient.get<Movie[]>(`/movie?name=${encodeURI(name)}&page=${page}&per_page=50`)
}


export let getNightById = (id: string) => {
    return apiClient.get<NightDetails>(`night/${id}`)
}

export let getPersonById = (id: string) => apiClient.get<PersonDetails>(`person/${id}`);