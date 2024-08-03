use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt::Debug;

// Custom error handler function
pub fn e500<T>(e: T) -> ErrorResponse
where
    T: Debug + std::fmt::Display + 'static,
{
    let error_message = format!("ERROR: {:?}", e);
    ErrorResponse::InternalServerError(error_message)
}

// Custom error response struct
#[derive(Debug, Clone)]
pub struct ErrorResponse {
    status_code: StatusCode,
    message: String,
}

impl ErrorResponse {
    #[allow(non_snake_case)]
    pub fn InternalServerError(message: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.status_code, self.message).into_response()
    }
}

impl From<tera::Error> for ErrorResponse {
    fn from(err: tera::Error) -> Self {
        ErrorResponse::InternalServerError(format!("Template rendering error: {:?}", err))
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error ({}): {}", self.status_code, self.message)
    }
}
