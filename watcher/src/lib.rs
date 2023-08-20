use anyhow::Result;
use sea_orm::DatabaseConnection;
use serde::de::DeserializeOwned;
use sqlx::postgres::PgListener;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;

pub async fn start_listening<'a, P, F, Fut>(
  pool: &'a Pool<Postgres>,
  db: &'a DatabaseConnection,
  workers: HashMap<&'a str, F>,
) -> Result<()>
where
  P: DeserializeOwned + Sized + Debug,
  Fut: Future<Output = Result<()>> + 'a,
  F: Fn(P, &'a DatabaseConnection) -> Fut,
{
  let mut listener = PgListener::connect_with(pool).await.unwrap();
  let channels: Vec<&str> = workers.keys().into_iter().map(|key| *key).collect();
  listener.listen_all(channels).await?;

  loop {
    while let Some(notification) = listener.try_recv().await? {
      let chanel = notification.channel();
      let call_back = workers.get(chanel).unwrap();

      let payload_string = notification.payload().to_owned();
      let payload = serde_json::from_str::<P>(&payload_string).unwrap();

      call_back(payload, db).await.unwrap();
    }
  }
}
