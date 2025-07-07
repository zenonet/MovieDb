use std::{fs, sync::Arc, time::Duration};

use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router
};
use chrono::{DateTime, Utc};
use serde::{Deserialize};
use sqlx::{pool, postgres::PgPoolOptions, PgPool};
use tokio::{net::TcpListener};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    fs::read_dir(".")
        .unwrap()
        .for_each(|d| println!("{}", d.unwrap().file_name().into_string().unwrap()));

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
        .route("/movie", post(post_movie))
        .route("/night", post(post_night))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:80".parse().unwrap(),   // vite dev server
                    "http://localhost:3000".parse().unwrap(), // rsbuild dev server
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



#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewMovieView{
    movie_id: Uuid,
    person_id: Uuid,
}

#[derive(Deserialize)]
struct NewNight{
    timestamp: DateTime<Utc>,
    description: Option<String>,

    movies: Vec<NewMovieView>,
}

async fn post_night(
    State(state): State<Arc<AppState>>,
    Json(night): Json<NewNight>
) -> Result<StatusCode, (StatusCode, String)>{

    let mut transation = state.db_pool.begin().await.unwrap();

    // insert night
    let night_id = Uuid::new_v4();
    sqlx::query!("INSERT INTO nights (id, time, description) VALUES ($1,$2,$3)",
        night_id,
        night.timestamp,
        night.description
    ).execute(&mut *transation).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // insert views
    for view in night.movies{ // Unfortunately, we can't do these in parallel when using a transaction
        let view_id = Uuid::new_v4();
        let res = sqlx::query!("INSERT INTO movie_views (id, movie_id, night_id, person_id) VALUES ($1,$2,$3,$4)",
            view_id,
            view.movie_id,
            night_id,
            view.person_id
        ).execute(&mut *transation).await;

        if let Err(e) = res{
            transation.rollback().await.unwrap();
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    }

    transation.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

/* async fn post_rating(
    State(state): State<Arc<AppState>>,
    Json(rating): Json<Rating>
) -> Result<StatusCode, (StatusCode, String)>{
    Ok(())
} */