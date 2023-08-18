use anyhow::Result;
use serde::de::DeserializeOwned;

pub async fn fetch_nft<T: DeserializeOwned>(
  token_address: &str,
  token_id: &str,
  http_client: &surf::Client,
) -> Result<T> {
  let mut res = http_client
    .get(format!("asset/{token_address}/{token_id}"))
    .await
    .unwrap();

  Ok(res.body_json::<T>().await.unwrap())
}
