use axum::{
  async_trait,
  extract::{FromRequest, FromRequestParts, Query},
  http::{self, request::Parts},
  Json,
};
use error::ValidateError;
use validator::Validate;

pub struct ValidatedQuery<T>(pub T);
pub struct ValidatedJson<J>(pub J);

#[async_trait]
impl<S, T> FromRequestParts<S> for ValidatedQuery<T>
where
  S: Send + Sync,
  T: Validate,
  Query<T>: FromRequestParts<S>,
{
  type Rejection = ValidateError;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let query = Query::<T>::from_request_parts(parts, state).await;

    if let Ok(Query(data)) = query {
      match data.validate() {
        Ok(_) => Ok(ValidatedQuery(data)),
        Err(err) => Err(ValidateError::from(err.to_string())),
      }
    } else {
      Err(ValidateError::from("Invalid query string"))
    }
  }
}

#[async_trait]
impl<S, B, J> FromRequest<S, B> for ValidatedJson<J>
where
  B: Send + 'static,
  S: Send + Sync,
  J: Validate,
  Json<J>: FromRequest<S, B>,
{
  type Rejection = ValidateError;

  async fn from_request(req: http::Request<B>, state: &S) -> Result<Self, Self::Rejection> {
    let json = Json::<J>::from_request(req, state).await;

    match json {
      Ok(Json(json_body)) => match json_body.validate() {
        Ok(_) => Ok(ValidatedJson(json_body)),
        Err(err) => Err(ValidateError::from(err.to_string())),
      },
      Err(_) => Err(ValidateError::from("Invalid json body")),
    }
  }
}
