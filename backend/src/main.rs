use std::{fs, sync::Arc, time::Duration};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
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
        .route("/movie/{name}", get(get_movie_by_name))
        .route("/movie", post(post_movie).get(get_movies))
        .route("/night", post(post_night))
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

async fn get_movie_by_name(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Movie>>, (StatusCode, String)> {
    let res = sqlx::query_as!(
        Movie,
        "SELECT * FROM movies WHERE UPPER(movies.name) LIKE UPPER($1) LIMIT $2 OFFSET $3",
        format!("%{}%", name),
        pagination.per_page as i32,
        (pagination.page * pagination.per_page) as i32
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(res))
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewMovieView {
    movie_id: Uuid,
    person_id: Uuid,
}

#[derive(Deserialize)]
struct NewNight {
    timestamp: Option<DateTime<Utc>>,
    description: Option<String>,

    movies: Vec<NewMovieView>,
}

async fn post_night(
    State(state): State<Arc<AppState>>,
    Json(night): Json<NewNight>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut transation = state.db_pool.begin().await.unwrap();

    // insert night
    let night_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO nights (id, time, description) VALUES ($1,$2,$3)",
        night_id,
        night.timestamp.unwrap_or_else(|| Utc::now()),
        night.description
    )
    .execute(&mut *transation)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // insert views
    for view in night.movies {
        // Unfortunately, we can't do these in parallel when using a transaction
        let view_id = Uuid::new_v4();
        let res = sqlx::query!(
            "INSERT INTO movie_views (id, movie_id, night_id, person_id) VALUES ($1,$2,$3,$4)",
            view_id,
            view.movie_id,
            night_id,
            view.person_id
        )
        .execute(&mut *transation)
        .await;

        if let Err(e) = res {
            transation.rollback().await.unwrap();
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    }

    transation
        .commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
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
) -> Result<StatusCode, (StatusCode, String)> {
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

    Ok(StatusCode::CREATED)
}
