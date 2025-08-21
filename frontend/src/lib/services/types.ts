


export interface Movie{
    id: string,
    name: string,
}


export interface Person{
    name: string
    id: string
}

export interface PersonWithRatings{
    name: string
    id: string
    avgRating: number,
    ratingCount: number,
}

export interface PersonDetails{
    name: string
    id: string
    latest_nights: Array<Night>
}

export interface Night{
    id: string
    time: Date
    movie?: Movie
    avgRating?: number
}

export interface NightDetails{
    id: string
    description?: string
    time: Date
    movie: Movie
    persons: Array<PersonWithRatings>
}

export interface MovieDetails{
    id: string
    name: string,
    tagline?: string,
    coverUrl?: string
    trailerUrl?: string,
    isMementoImport: boolean
    description?: string
    yearOfPublication?: number
    nights: Array<Night>
    avgRating?: number,
    actors?: string,
    // Runtime in minutes
    duration?: number,
}

export interface WatchlistEntry{
    movie: Movie,
    idx: number
}
export interface Watchlist{
    name: string,
    id: string,
}

export interface WatchlistDetails{
    name: string
    id: string
    description?: string,
    entries: Array<WatchlistEntry>
}

export interface Rating{
    person: Person
    value: number
    time: Date
}