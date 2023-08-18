use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use database::{nft, prelude::Nft};
use sea_orm::{sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn inactive_nfts(db: &DatabaseConnection) -> Result<()> {
  let now = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis());

  Nft::update_many()
    .col_expr(nft::Column::IsActive, Expr::value(false))
    .filter(nft::Column::EndDate.lt(now))
    .exec(db)
    .await?;

  Ok(())
}
