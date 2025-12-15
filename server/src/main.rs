use std::error::Error;
use sqlx::{Row, PgPool};
use sqlx::postgres;

use axum::{routing::{get, post},  Router};

mod get;
mod post;

type SimpleResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;


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
    let url = env::var("DATABASE_URL").unwrap();
    
    /// for startup 
    let pool = sqlx::postgres::PgPoolOptions::new().connect(&url).await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    /// create state for pool and has to be owned so clone it 
    let state = ConnectionState{db: pool.clone()};

    ///creating router for Axum for chatroom HTTP methods; https://docs.rs/axum/latest/axum/routing/struct.Router.html
    let app = Router::new().route("/health", get(health_check))
        /// may only need this route for post and get 
        .route("/messages", post(create_message).get(get_all_messages))
        .with_state(pool.clone());

    //to run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
