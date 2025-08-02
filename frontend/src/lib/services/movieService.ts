import { PUBLIC_API_URL } from "$env/static/public";
import axios from "axios";
import type { Movie, MovieDetails, NightDetails, Person, PersonDetails } from "./types";

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

export let getPersonsByName = (name: string, page:number = 0) => apiClient.get<Person[]>(`person?name=${encodeURI(name)}&page=${page}&per_page=15`)


export let postNight = async (movie: Movie, description: string | null, personIds: Array<string>) => {
    const resp = await apiClient.post("night", {
        time: new Date().toISOString(),
        description,
        persons: personIds,
        movie: movie.id
    });

    const personToViewMap: Map<string, string> = new Map(Object.entries(resp.data));
    return personToViewMap
}

export let postRating = async (viewId: string, rating: number) => {
    const resp = await apiClient.post("rating", {
        value: rating,
        viewId,
        time: new Date().toISOString()
    });

    const ratingId:string = resp.data;
    return ratingId;
}