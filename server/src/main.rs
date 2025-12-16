use sqlx::{PgPool};
// use sqlx::postgres;

use dotenv::dotenv;
use std::env;
use tower_http::cors::CorsLayer;

use axum::{Router, routing::post};


mod routes;
use routes::{post::post_method, get::get_all_messages};
mod schema;

// type SimpleResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;


/// struct for connection state pool for api methods
#[derive(Debug, Clone)]
struct ConnectionState {
    pub db: PgPool,
}


/// taken from https://github.com/launchbadge/sqlx
/// sqlx is used to manager our database 
/// axum is working for our https method communicating between the backend and the database
/// connects to database and creates our router
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").unwrap();
    
    // for startup 
    let pool = sqlx::postgres::PgPoolOptions::new().connect(&url).await?;

    // check for connection
    // sqlx::query("Select 1").execute(&pool).await?;

    // println!("Database connection good");
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    // create state for pool and has to be owned so clone it 
    let state = ConnectionState{db: pool.clone()};

    let allowed_origins = env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);
    
    //creating router for Axum for chatroom HTTP methods; https://docs.rs/axum/latest/axum/routing/struct.Router.html
    let app = Router::new()
        // .route("/health", get(health_check))
        // may only need this route for post and get 
        .route("/messages", post(post_method).get(get_all_messages))
        .layer(cors)
        .with_state(pool.clone());



    let port = std::env::var("PORT").unwrap();
    let addr = format!("0.0.0.0:{}", port);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();


    Ok(())

}