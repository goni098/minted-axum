use crate::intercept::security::Guard;
use crate::intercept::validate::{ValidatedJson, ValidatedQuery};
use crate::opensea::fetch_nft;
use anyhow::Result;
use axum::{http::StatusCode, Json};
use chrono::{Local, NaiveDateTime};
use database::collection;
use database::{
  nft,
  prelude::{Collection, Nft},
};
use error::AppError;
use minted_axum_api::{HttpClient, Postgres};
use rand::Rng;
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm::{ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Deserialize, IntoParams, Validate, Serialize)]
#[into_params(parameter_in = Query)]
pub struct ParseNftQuery {
  token_id: String,
  token_address: String,
}

#[utoipa::path(
  get,
  path = "/nfts/parsing",
  params(
    ParseNftQuery
  ),
  tag = "nfts",
  responses(
      (status = 200, description = "return nft metadata")
  )
)]
pub async fn parse_nft(
  HttpClient(http_client): HttpClient,
  ValidatedQuery(query): ValidatedQuery<ParseNftQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
  let nft_res = fetch_nft(&query.token_address, &query.token_id, &http_client)
    .await
    .unwrap();
  Ok(Json(nft_res))
}

#[derive(Deserialize, ToSchema, Validate, Serialize, Debug)]
pub struct CreateNft {
  token_id: String,
  token_address: String,
  time: i64,
}
// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2OTI0OTgwOTAsImlkIjozLCJhZGRyZXNzIjoiMHgyNTU1QjkwY2VjYTg4RGZGQzI2NUE2MTMzNGE4MEFCOTljNWMzRDUxIiwiaXNfYWRtaW4iOmZhbHNlfQ.XjATk3CbO-wq-BLSjRm_NzL2fAiulaROHCvL_1bL7_I

#[utoipa::path(
  post,
  path = "/nfts",
  request_body(content = CreateNft, description = "nft create input", content_type = "application/json"),
  tag = "nfts",
  responses(
      (status = 201, description = "create nft")
  ),
  security(
    ("BearerAuth" = []),
  )
)]
pub async fn create_nft(
  Postgres(db): Postgres,
  Guard(claims): Guard,
  HttpClient(http_client): HttpClient,
  ValidatedJson(body): ValidatedJson<CreateNft>,
) -> Result<(StatusCode, String), AppError> {
  let CreateNft {
    token_id,
    token_address,
    time,
  } = body;

  let parse_query = ParseNftQuery {
    token_id,
    token_address,
  };

  let nft_res = fetch_nft::<NFTFetched>(
    &parse_query.token_address,
    &parse_query.token_id,
    &http_client,
  )
  .await
  .unwrap();

  let NFTFetched {
    name,
    image_url,
    permalink,
    description,
    image_thumbnail_url,
    collection,
  } = nft_res;

  let exsited_collection = Collection::find()
    .filter(collection::Column::Slug.eq(&collection.slug))
    .one(&db)
    .await?;

  let collection_id = if let Some(col) = exsited_collection {
    col.id
  } else {
    Collection::insert(collection::ActiveModel {
      slug: Set(collection.slug),
      name: Set(collection.name),
      image_url: Set(collection.image_url),
      description: Set(collection.description),
      ..Default::default()
    })
    .exec(&db)
    .await?
    .last_insert_id
  };

  let price_payment = rand::thread_rng().gen_range(1.0..=100.0);
  let square_price = price_payment / (time as f64);

  Nft::insert(nft::ActiveModel {
    token_address: Set(parse_query.token_address),
    token_id: Set(parse_query.token_id.clone()),
    name: Set(name.unwrap_or(format!("#{}", parse_query.token_id))),
    original_url: Set(permalink),
    image_url: Set(image_url),
    collection_id: Set(collection_id),
    is_active: Set(true),
    user_id: Set(claims.id),
    description: Set(description),
    thumbnail_url: Set(image_thumbnail_url),
    square_price: Set(square_price as f64),
    end_date: Set(
      NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis() + time * 3600 * 1000)
        .unwrap(),
    ),
    ..Default::default()
  })
  .exec(&db)
  .await?;

  Ok((StatusCode::CREATED, String::from("created")))
}

#[derive(Deserialize)]
struct NFTFetched {
  name: Option<String>,
  image_url: String,
  permalink: String,
  description: Option<String>,
  image_thumbnail_url: Option<String>,
  collection: CollectionFetched,
}

#[derive(Deserialize)]
struct CollectionFetched {
  slug: String,
  name: String,
  description: Option<String>,
  image_url: Option<String>,
}
