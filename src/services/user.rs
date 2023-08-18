// use crate::intercept::sercurity::Guard;
// use axum::Json;
use error::AppError;
use minted_axum_api::Postgres;
use sea_orm::FromQueryResult;
use serde::Serialize;

// use error::AppError;
// use crate::database::{model::User, schema::user};

#[derive(Debug, Serialize, FromQueryResult)]
pub struct RateBusines {
  pub valuer_id: i32,
  pub business_id: i32,
  pub rating: i32,
}

// #[axum_macros::debug_handler]
#[utoipa::path(
  get,
  path = "/users",
  tag = "user",
  responses(
      (status = 200, description = "return your information")
  ),
  security(
    ("BearerAuth" = []),
  )
)]
pub async fn who_am_i(Postgres(_conn): Postgres) -> Result<String, AppError> {
  Ok("fsa".into())
}
