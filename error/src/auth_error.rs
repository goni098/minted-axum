use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;

pub enum AuthError {
  WrongCredentials,
  MissingCredentials,
  ExpriedCredentials,
  WrongSignature,
}

impl IntoResponse for AuthError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
      AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
      AuthError::ExpriedCredentials => (StatusCode::UNAUTHORIZED, "Expried credentials"),
      AuthError::WrongSignature => (StatusCode::UNAUTHORIZED, "Invalid signature"),
    };
    let body = Json(json!({
        "error": error_message,
    }));
    (status, body).into_response()
  }
}
