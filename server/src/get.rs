use crate::SimpleResult;
use axum::{Router, response::IntoResponse, routing::get};
use sqlx::Row;

// Followed the structure found at https://www.youtube.com/watch?v=FDWKlJmHv6k

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

// Does a simple check on if the server is running properly. Otherwise, return an error
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

// Returns a Vector containing the users present in rows
async fn user_list(app: Router) -> Result<Vec<Row>, ApiError> {
    let result = sqlx::query(&app).fetch_all().await.unwrap();
    result
    // returns the vector of all users
}

// Returns a Json containing full message history
async fn message_history(mid: u32, app: Router) -> Result<Row, ApiError> {
    let result = sqlx::query(&app).fetch_one(mid).await.unwrap();
    result
    // returns a whole message line of mid
}

// Returns a user with the given id 
async fn get_user(Path(id): Path<u32>) -> Result<Json<Value>, ApiError> {
    if id > 100 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(json!({"id": id, "name": "User"})))
}

pub fn get_router(router: Router) -> String {
    router.route("/health", get(health_check))
}