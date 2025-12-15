/// https://docs.rs/axum/latest/axum/extract/struct.State.html

/// https://www.youtube.com/watch?v=7RlVM0D4CEA explanation of taking json payload from client to store into database
/// for put method 

///extract state from Axum 
use axum::{extract::State, Json};

/// create query to be sent to db
use sqlx::query; 
use sqlx::PgPool;

use crate::{schema::{User, Message}};



pub async fn get(State(pool): State<PgPool>, Json(message): Json<Message>) -> Json<Message> {
    let message = query!("SELECT * FROM messages ORDER BY created_at DESC").fetch_all(&ConnectionState.db).await().unwrap();

    Json(message)
}