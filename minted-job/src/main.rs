mod jobs;

use cronjob::Scheduler;
use dotenv::dotenv;
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() {
  env::set_var("RUST_LOG", "debug");
  dotenv().ok();

  let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
  let pg_conn = Database::connect(db_url)
    .await
    .expect("fail to connect database");

  Scheduler::new()
    .set_context(pg_conn)
    .job("* * * * * *", &|db| {
      Box::pin(async move {
        println!("Every second!");
        jobs::inactive_nfts(&db).await.unwrap();
      })
    })
    .start()
    .await
    .unwrap_or_else(|err| {
      eprintln!("An error occured: {}", err);
    });
}
