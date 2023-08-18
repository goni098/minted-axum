use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Internal server error: {}", self.0),
    )
      .into_response()
  }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
// Example
// async fn handler() -> Result<(), AppError> {
//     try_thing()?;
//     Ok(())
// }
impl<E> From<E> for AppError
where
  E: Into<anyhow::Error>,
{
  fn from(err: E) -> Self {
    Self(err.into())
  }
}
