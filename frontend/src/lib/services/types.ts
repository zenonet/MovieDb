


export interface Movie{
    id: string,
    name: string,
}


export interface Person{
    name: string
    id: string
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
}

export interface NightDetails{
    id: string
    description?: string
    time: Date
    movie: Movie
    persons: Array<Person>
}

export interface MovieDetails{
    id: string
    name: string,
    tagline?: string,
    coverUrl?: string
    description?: string
    yearOfPublication?: number
    nights: Array<Night>
}