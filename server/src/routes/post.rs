/// https://docs.rs/axum/latest/axum/extract/struct.State.html

/// https://www.youtube.com/watch?v=7RlVM0D4CEA explanation of taking json payload from client to store into database
/// for put method 

///extract state from Axum 
use axum::{extract::State, Json};

/// create query to be sent to db
use sqlx::query; 
use sqlx::PgPool;

use crate::{schema::{User, Message}};

///pool should be if connected to database
pub async fn post(State(pool): State<PgPool>, Json(message): Json<Message>) -> Json<Message> {
    // https://www.shuttle.dev/blog/2023/10/04/sql-in-rust for quering into db 
    let result = query!(
            "INSERT INTO messages (username, content, created_at) 
            VALUES ($1, $2, $3) 
            RETURNING id, username, content, created_at", 
            message.username, 
            message.content, 
            message.created_at
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        Json(Message {
            username: result.username,
            content: result.content,
            created_at: result.created_at,
        })
}