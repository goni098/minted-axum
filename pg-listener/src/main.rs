mod img;
mod listenner;

use dotenv::dotenv;
use img::generate_9_block_images;
// use pg_listener::{start_listening, start_trigger};
use sea_orm::Database;

#[tokio::main]
async fn main() {
  // dotenv().ok();
  // let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
  // let pool = sqlx::PgPool::connect(&db_url).await.unwrap();

  // let channels = vec!["nfts_change"];

  // start_trigger(&pool).await.unwrap();

  // start_listening(&pool, channels, listenner::update_nfts_poistion)
  //   .await
  //   .unwrap();

  dotenv().ok();

  let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
  let pg_conn = Database::connect(db_url)
    .await
    .expect("fail to connect database");

  generate_9_block_images(&pg_conn)
    .await
    .unwrap_or_else(|e| eprintln!("error: {}", e));
}
