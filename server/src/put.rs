/// https://docs.rs/axum/latest/axum/extract/struct.State.html

/// https://www.youtube.com/watch?v=7RlVM0D4CEA explanation of taking json payload from client to store into database
/// for put method 

///extract state from Axum 
use axum::{extract::State, Json};

/// create query to be sent to db
use sqlx::query; 

use crate::{}

///pool should be if connected to database
pub async fn post(State(pool): State<Pool>, Json(message): Json<Message>) -> Json<Message> {
    /// https://www.shuttle.dev/blog/2023/10/04/sql-in-rust for quering into db 
    query!(message.username, message.content, message.created_at).execute(&pool).await.unwrap();
}