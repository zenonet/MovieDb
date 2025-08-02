use std::{collections::HashMap, fs, sync::Arc, time::Duration};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    //expose environment variables from .env file
    dotenvy::dotenv().expect("Unable to access .env file");

    //set variables from enviroment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("0.0.0.0:80".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    //create our database pool
    let db_pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    // Migrate db to the latest version
    let migrator = sqlx::migrate!();
    migrator
        .run(&db_pool)
        .await
        .expect("Failed to migrate database");
    println!("Migrations completed successfully");

    //create our tcp listener
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create tcp listener");

    println!("listening on http://{}", listener.local_addr().unwrap());

    let state = AppState { db_pool };
    let state = Arc::new(state);

    // compose the routes
    let app = Router::new()
        .route(
            "/",
            get(|| async { "Connection to moviedb API is working!" }),
        )
        .route("/movie/{id}", get(get_movie_details))
        .route("/movie", post(post_movie).get(get_movies))
        .route("/night", post(post_night))
        .route("/night/{id}", get(get_night_details))
        .route("/person", get(get_persons))
        .route("/person/{id}", get(get_person_details))
        .route("/rating", post(post_rating))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:80".parse().unwrap(),   // vite dev server
                    "http://localhost:5173".parse().unwrap(), // rsbuild dev server
                    "https://zenonet.de".parse().unwrap(),
                ])
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

#[derive(Deserialize)]
struct NewMovie {
    name: String,
}

async fn post_movie(
    State(state): State<Arc<AppState>>,
    Json(movie): Json<NewMovie>,
) -> Result<StatusCode, (StatusCode, String)> {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO movies (id, name) VALUES ($1,$2)",
        id,
        movie.name,
    )
    .execute(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

#[derive(Serialize, sqlx::FromRow, Default)]
struct Movie {
    id: Uuid,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Pagination {
    page: i32,
    per_page: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 0,
            per_page: 10,
        }
    }
}

#[derive(Deserialize)]
struct MovieFetchArgs {
    page: i32,
    per_page: i32,

    name: Option<String>,
}

#[derive(Serialize)]
struct NightStub{
    id: Uuid,
    time: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NigthStubWithRating{
    id: Uuid,
    time: DateTime<Utc>,
    avg_rating: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MovieDetails{
    id: Uuid,
    name: String,
    tagline: Option<String>,
    cover_url: Option<String>,
    description: Option<String>,
    year_of_publication: Option<i32>,
    nights: Vec<NigthStubWithRating>,
    avg_rating: f64,
}

async fn get_movie_details(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<MovieDetails>, (StatusCode, String)> {
    let movie = sqlx::query!(
        "SELECT * FROM movies WHERE id = $1",
        id
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(movie) = movie else{
        return Err((StatusCode::BAD_REQUEST, String::from("A movie with that id does not exist")));
    };

    // Fetch movie nights showing this movie

    let nights = sqlx::query_as!(NigthStubWithRating, "
SELECT nights.id, nights.time, AVG(ratings.value) AS avg_rating FROM nights
JOIN movie_views ON movie_views.night_id=nights.id
JOIN ratings ON ratings.movie_view_id=movie_views.id
WHERE movie_id=$1
GROUP BY nights.id
",
        id
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Get the average rating of this movie across all views

    // NOTE: This takes the average of all ratings including second ratings of a single movie view (aka. hangover ratings)
    let avg_rating = sqlx::query_scalar!("
SELECT AVG(ratings.value) FROM ratings
JOIN movie_views ON ratings.movie_view_id=movie_views.id
JOIN nights ON nights.id=movie_views.night_id
WHERE movie_id=$1
", id).fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?.unwrap();

    let details = MovieDetails{
        name: movie.name,
        id,
        nights,
        tagline: movie.tagline,
        cover_url: movie.cover_url,
        description: movie.description,
        year_of_publication: movie.year_of_publication,
        avg_rating
    };

    Ok(Json(details))
}

async fn get_movies(
    State(state): State<Arc<AppState>>,
    Query(args): Query<MovieFetchArgs>,
) -> Result<Json<Vec<Movie>>, (StatusCode, String)> {
    let mut query = String::from("SELECT * FROM movies");

    if args.name.is_some() {
        query.push_str(" WHERE UPPER(name) LIKE $1");
    }

    query.push_str(" LIMIT $2 OFFSET $3");

    let mut q = sqlx::query_as::<_, Movie>(&query);

    if let Some(name) = args.name {
        q = q.bind(format!("%{}%", name.to_uppercase()));
    }

    q = q
        .bind(args.per_page as i32)
        .bind((args.page * args.per_page) as i32);


    let res = q.fetch_all(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(res))
}


async fn get_persons(
    State(state): State<Arc<AppState>>,
    Query(args): Query<MovieFetchArgs>
) -> Result<Json<Vec<PersonStub>>, (StatusCode, String)>{
        let mut query = String::from("SELECT * FROM persons");

    if args.name.is_some() {
        query.push_str(" WHERE UPPER(name) LIKE $1");
    }

    query.push_str(" LIMIT $2 OFFSET $3");

    let mut q = sqlx::query_as::<_, PersonStub>(&query);

    if let Some(name) = args.name {
        q = q.bind(format!("%{}%", name.to_uppercase()));
    }

    q = q
        .bind(args.per_page as i32)
        .bind((args.page * args.per_page) as i32);


    let res = q.fetch_all(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(res))
}

#[derive(Deserialize)]
struct NewNight {
    time: Option<DateTime<Utc>>,
    description: Option<String>,

    persons: Vec<Uuid>,
    movie: Uuid,
}

async fn post_night(
    State(state): State<Arc<AppState>>,
    Json(night): Json<NewNight>,
) -> Result<Json<HashMap<Uuid, Uuid>>, (StatusCode, String)> {
    let mut transation = state.db_pool.begin().await.unwrap();

    // insert night
    let night_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO nights (id, movie_id, time, description) VALUES ($1,$2,$3,$4)",
        night_id,
        night.movie,
        night.time.unwrap_or_else(|| Utc::now()),
        night.description
    )
    .execute(&mut *transation)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;


    let mut person_to_view_map = HashMap::<Uuid, Uuid>::with_capacity(night.persons.len());

    // insert views
    for person in night.persons {
        // Unfortunately, we can't do these in parallel when using a transaction
        let view_id = Uuid::new_v4();
        let res = sqlx::query!(
            "INSERT INTO movie_views (id, night_id, person_id) VALUES ($1,$2,$3)",
            view_id,
            night_id,
            person
        )
        .execute(&mut *transation)
        .await;

        if let Err(e) = res {
            transation.rollback().await.unwrap();
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }

        person_to_view_map.insert(person, view_id);
    }

    transation
        .commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(person_to_view_map))
}


#[derive(Serialize, FromRow)]
struct PersonStub{
    id: Uuid,
    name: String,
}


#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
struct PersonStubWithRating{
    id: Uuid,
    name: String,
    avg_rating: Option<f64>,
    rating_count: Option<i64>,
}

#[derive(Serialize)]
struct MovieStub{
    id: Uuid,
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NightDetails{
    id: Uuid,
    description: Option<String>,
    time: DateTime<Utc>,
    movie: MovieStub,
    persons: Vec<PersonStubWithRating>
}

async fn get_night_details(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>
) -> Result<Json<NightDetails>, (StatusCode, String)>{
    let night = sqlx::query!("SELECT time, nights.description, movies.name AS movie_name, movies.id AS movie_id FROM nights JOIN movies ON movie_id = movies.id WHERE nights.id=$1", id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(night) = night else {
        return Err((StatusCode::NOT_FOUND, String::from("No movie with that id found")));
    };

    // Fetch persons
    let persons = sqlx::query_as!(PersonStubWithRating, "SELECT persons.name, persons.id, AVG(ratings.value) AS avg_rating, COUNT(ratings.id) AS rating_count FROM movie_views JOIN persons ON persons.id = movie_views.person_id JOIN ratings ON ratings.movie_view_id=movie_views.id WHERE night_id=$1 GROUP BY persons.id", id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(
        NightDetails{
            id,
            time: night.time,
            description: night.description,
            movie: MovieStub { id: night.movie_id, name: night.movie_name },
            persons
        }
    ))
}


#[derive(Serialize)]
struct NightStubWithMovie{
    #[serde(flatten)]
    stub: NightStub,
    movie: MovieStub,
}

#[derive(Serialize)]
struct PersonDetails{
    name: String,
    id: Uuid,
    latest_nights: Vec<NightStubWithMovie>
}

async fn get_person_details(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>
) -> Result<Json<PersonDetails>, (StatusCode, String)>{
    let person = sqlx::query!("SELECT * FROM persons WHERE id=$1", id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(person) = person else{
        return Err((StatusCode::NOT_FOUND, String::from("No person with that id found")))
    };

    // Fetch last nights
    let latest_nights = sqlx::query!("SELECT night_id, nights.time, movies.name as movie_name, movies.id as movie_id FROM movie_views JOIN nights ON movie_views.night_id = nights.id JOIN movies ON nights.movie_id = movies.id WHERE movie_views.person_id = $1 ORDER BY nights.time LIMIT 10", id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;


    Ok(Json(
        PersonDetails{
            name: person.name,
            id: person.id,
            latest_nights: latest_nights.into_iter().map(|n| NightStubWithMovie{
                movie: MovieStub { id: n.movie_id, name: n.movie_name },
                stub: NightStub { id: n.night_id, time: n.time }
            }).collect::<Vec<NightStubWithMovie>>()
        }
    ))
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewRating {
    value: f64,
    view_id: Uuid,
    time: Option<DateTime<Utc>>,
}

async fn post_rating(
    State(state): State<Arc<AppState>>,
    Json(rating): Json<NewRating>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rating_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO ratings (id,movie_view_id,time,value) VALUES ($1,$2,$3,$4)",
        rating_id,
        rating.view_id,
        rating.time.unwrap_or_else(|| Utc::now()),
        rating.value
    )
    .execute(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, rating_id.to_string()))
}
