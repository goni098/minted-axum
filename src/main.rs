mod intercept;
mod open_api;
mod opensea;
mod services;
mod utils;

use axum::{
  routing::{get, post},
  Router,
};
use dotenv::dotenv;
use minted_axum_api::AppState;
use open_api::ApiDoc;
// use std::env;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
  // env::set_var("RUST_LOG", "debug");
  // tracing_subscriber::fmt::init();
  dotenv().ok();

  let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
  let opensea_api_key = std::env::var("OPENSEA_API_KEY").expect("api key must be set");

  let app = Router::new()
    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .route("/auth/nonce", get(services::auth::get_nonce))
    .route("/auth/login", post(services::auth::login))
    .route("/users", get(services::user::who_am_i))
    .route("/nfts/parsing", get(services::nft::parse_nft))
    .route("/nfts", post(services::nft::create_nft))
    .layer(
      CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any),
    )
    .with_state(AppState::new(&db_url, &opensea_api_key).await.unwrap());

  // run it with hyper on localhost:8080
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

// Minnie
