use crate::SimpleResult;
use axum::{Router, response::IntoResponse, routing::get};

#[derive(Debug)]
enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalErr,
}

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

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

async fn user_list() -> Result<???, ApiError> {
    todo!();
}

pub fn get_router(router: Router) -> String {
    router.route("/health", get(health_check))
        .route("/users", get(list_users))
}