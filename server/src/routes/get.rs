// // https://docs.rs/axum/latest/axum/extract/struct.State.html
// https://docs.rs/sqlx-build-trust/latest/sqlx_build_trust/fn.query_as.html

// // https://www.youtube.com/watch?v=7RlVM0D4CEA explanation of taking json payload from client to store into database
// // for put method 

// // ///extract state from Axum 
// use axum::{extract::State, Json};

// // /// create query to be sent to db
// use sqlx::query_as; 
// use sqlx::PgPool;

// use crate::{schema::{User, Message}};


// query! for database time connection at compile time vs after b/c of .env 
// pub async fn get_method(State(pool): State<PgPool>, Json(message): Json<Message>) -> Json<Message> {
//     // use query_as returns string value 
//     let res = query_as("SELECT content FROM messages ORDER BY created_at DESC LIMIT 1").fetch_one(&pool).await.unwrap();

//     Json(Message {
//             content: res,
//         })
// }