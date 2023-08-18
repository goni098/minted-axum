use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn update_nfts_poistion<'a>(
  payload: serde_json::Value,
  pool: &'a Pool<Postgres>,
) -> Result<()> {
  dbg!(payload);
  sqlx::query!(
    r#"
      WITH nft_ext AS (
        SELECT
        "nft"."id",
        row_number() OVER (
          ORDER BY "nft"."square_price" DESC NULLS LAST
        ) AS "index"
        FROM "nft"
        WHERE "nft"."is_active" = true
      )
      UPDATE nft
      SET
        "last_position" = "position",
        "position" = (
          SELECT "nft_ext"."index"
          FROM "nft_ext"
          WHERE "nft"."id" = "nft_ext"."id"
          LIMIT 1
        )
      WHERE is_active = true
  "#
  )
  .execute(pool)
  .await?;
  Ok(())
}
