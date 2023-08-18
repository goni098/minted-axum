use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};

pub struct ValidateError(String);

impl<E> From<E> for ValidateError
where
  E: Into<String>,
{
  fn from(err: E) -> Self {
    Self(err.into())
  }
}

impl IntoResponse for ValidateError {
  fn into_response(self) -> Response {
    (StatusCode::BAD_REQUEST, self.0).into_response()
  }
}
