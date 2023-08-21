use anyhow::Result;
use database::{nft, prelude::Nft};
use futures::future;
use image::io::Reader as ImageReader;
use image::{GenericImage, ImageBuffer};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::io::Cursor;

type ImageBuf = Vec<u8>;

async fn img_url_to_buffer(url: impl Into<String>) -> Result<ImageBuf, surf::Error> {
  let mut response = surf::get(url.into()).await?;

  let img_bytes = response.body_bytes().await?;

  let dynamic_image = ImageReader::new(Cursor::new(img_bytes))
    .with_guessed_format()?
    .decode()?;

  let mut bytes: ImageBuf = Vec::new();

  dynamic_image
    .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
    .unwrap();

  Ok(bytes)
}

fn nfts_range_of_block(block: u32) -> (u32, u32) {
  let start_position = 1 + (1..block).map(|i| i.pow(2)).sum::<u32>();
  let end_position: u32 = (1..=block).map(|i| i.pow(2)).sum::<u32>();
  (start_position, end_position)
}

pub async fn generate_9_block_images(db: &DatabaseConnection) -> Result<()> {
  const BLOCK_SIZE: u32 = 80;
  for block in 1..=9 {
    let db = db.clone();
    tokio::spawn(async move {
      let (start_position, end_position) = nfts_range_of_block(block);

      let childs_image_handles = Nft::find()
        .filter(nft::Column::IsActive.eq(true))
        .filter(nft::Column::Position.between(start_position, end_position))
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|nft| tokio::spawn(async { img_url_to_buffer(nft.image_url).await.unwrap() }));

      let childs_image_buff = future::join_all(childs_image_handles)
        .await
        .into_iter()
        .map(|h| h.unwrap())
        .collect::<Vec<ImageBuf>>();

      let mut image_block = ImageBuffer::new(BLOCK_SIZE, BLOCK_SIZE);
      let child_image_size: u32 = BLOCK_SIZE / block;

      for (idx, buf) in childs_image_buff.into_iter().enumerate() {
        let child_image = image::load_from_memory(&buf).unwrap().resize_exact(
          child_image_size as u32,
          child_image_size as u32,
          image::imageops::FilterType::Lanczos3,
        );

        let x: u32 = ((idx as u32) % block) * (BLOCK_SIZE / block);
        let y: u32 = ((idx as u32) / block) * (BLOCK_SIZE / block);

        image_block
          .copy_from(&child_image, x, y)
          .unwrap_or_else(|e| eprintln!("e: {}", e));
      }

      image_block.save(format!("/home/theanh098/rust-lang/rust-workspace/minted-axum-api/watcher/images/block_{block}.png")).unwrap();
      println!("done block {}", block);
    });
  }

  Ok(())
}
