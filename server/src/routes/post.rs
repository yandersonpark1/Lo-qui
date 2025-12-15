// https://docs.rs/axum/latest/axum/extract/struct.State.html

// https://www.youtube.com/watch?v=7RlVM0D4CEA explanation of taking json payload from client to store into database
// for put method 

// ///extract state from Axum 
use axum::{extract::State, Json};

// /// create query to be sent to db
use sqlx::query; 
use sqlx::PgPool;

use crate::{schema::{InitMessage, Message}};

// ///pool should be if connected to database
pub async fn post_method(State(pool): State<PgPool>, Json(message): Json<InitMessage>) -> Json<Message> {
    // https://www.shuttle.dev/blog/2023/10/04/sql-in-rust for quering into db 
    let result = query(
            "INSERT INTO messages (username, content) 
            VALUES ($1, $2) 
            RETURNING id, username, content, created_at")
            .bind(&message.username)
            .bind(&message.content)
            // .bind(&message.created_at)
        .fetch_one(&pool)
        .await
        .unwrap();

        Json(Message {
            content: message.content,
        })
}