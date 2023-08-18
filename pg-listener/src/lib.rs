use anyhow::Result;
use serde::de::DeserializeOwned;
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use std::fmt::Debug;
use std::future::Future;

pub async fn start_listening<'a, T, F, Fut>(
  pool: &'a Pool<Postgres>,
  channels: Vec<&str>,
  call_back: F,
) -> Result<(), Error>
where
  T: DeserializeOwned + Sized + Debug,
  Fut: Future<Output = Result<()>> + 'a,
  F: Fn(T, &'a Pool<Postgres>) -> Fut,
{
  let mut listener = PgListener::connect_with(pool).await.unwrap();
  listener.listen_all(channels).await?;
  loop {
    while let Some(notification) = listener.try_recv().await? {
      let payload_string = notification.payload().to_owned();
      let payload: T = serde_json::from_str::<T>(&payload_string).unwrap();
      call_back(payload, pool).await.unwrap();
    }
  }
}

pub async fn start_trigger(pool: &Pool<Postgres>) -> Result<()> {
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
