mod block;
mod position;

use anyhow::Result;
use block::generate_9_block_images;
use dotenv::dotenv;
use position::update_nfts_poisition;
use sea_orm::Database;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use watcher::start_listening;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();
  let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
  let pool = sqlx::PgPool::connect(&db_url).await.unwrap();
  let db = Database::connect(&db_url).await.unwrap();

  start_trigger(&pool).await.unwrap();

  let mut workers = HashMap::new();

  workers.insert("nfts_change", |payload: serde_json::Value, db| async move {
    update_nfts_poisition(db, payload)
      .await
      .unwrap_or_else(|e| eprint!("update_nfts_poisition  fail with err {}", e));

    generate_9_block_images(db)
      .await
      .unwrap_or_else(|e| eprint!("generate_block fail with err {}", e));
    Ok(())
  });

  start_listening(&pool, &db, workers).await
}

async fn start_trigger(pool: &Pool<Postgres>) -> Result<()> {
  sqlx::query!(
    r#"
      CREATE OR REPLACE FUNCTION nfts_change_listener()
      RETURNS TRIGGER AS $$
      BEGIN
        IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
          PERFORM pg_notify('nfts_change', row_to_json(NEW)::text);
        ELSE          
          PERFORM pg_notify('nfts_change', row_to_json(OLD)::text);
        END IF;
        RETURN NEW;
      END;
      $$ LANGUAGE plpgsql;
  "#
  )
  .execute(pool)
  .await?;

  sqlx::query!(
    r#"
      CREATE OR REPLACE TRIGGER nfts_change 
      AFTER 
        INSERT 
        OR DELETE 
        OR UPDATE OF square_price, is_active
      ON nft
      FOR EACH ROW 
      EXECUTE PROCEDURE nfts_change_listener();
  "#
  )
  .execute(pool)
  .await?;

  Ok(())
}
