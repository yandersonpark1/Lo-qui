use crate::SimpleResult;
use axum::{
    Router, 
    response::IntoResponse, 
    routing::get,
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;

#[derive(Debug)]
enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalErr,
}

// Create A set list of possible errors to be found given the related issue in the server
// ApiError::NotFound : Attempted to draw from something that does not exist
// ApiError::InvalidInput(error) : Attempted to input something the server does not understand
// ApiError::InternalErr : Internal Server Error (for example, not running)
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
            ApiError::InvalidInput(error) => (StatusCode::BAD_REQUEST, error),
            ApiError::InternalErr => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

#[derive(Clone)]
struct AppState {
    db: Pool<Postgres>,
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

// GET all messages
async fn get_all_messages(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let rows = sqlx::query("SELECT * FROM messages ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|_| ApiError::InternalErr)?;
    
    let messages: Vec<Value> = rows.iter().map(|row| {
        json!({
            "id": row.get::<i32, _>("id"),
            "username": row.get::<String, _> ("username"),
            "content": row.get::<String, _>("content"),
            "created_at": row.get::<chrono::NaiveDateTime, _>("created_at"),
        })
    }).collect();

    Ok(Json(json!({ "messages": messages })))
}

pub fn get_router(db: Pool<Postgres>) -> Router {
    let state = Arc::new(AppState { db });
    
    Router::new()
        .route("/health", get(health_check))
        .route("/messages", get(get_all_messages))
        .with_state(state)
}