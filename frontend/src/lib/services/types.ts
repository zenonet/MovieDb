


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
    description?: string
    yearOfPublication?: number
    nights: Array<Night>
    avgRating?: number
}

export interface WatchlistEntry{
    movie: Movie,
    idx: number
}

export interface Watchlist{
    name: string
    id: string
    description?: string,
    entries: Array<WatchlistEntry>
}