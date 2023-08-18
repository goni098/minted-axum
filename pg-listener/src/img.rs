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
  for block in 1..=9 {
    let (start_position, end_position) = nfts_range_of_block(block);

    let bytes_handles = Nft::find()
      .filter(nft::Column::IsActive.eq(true))
      .filter(nft::Column::Position.between(start_position, end_position))
      .all(db)
      .await?
      .into_iter()
      .map(|nft| tokio::spawn(async { img_url_to_buffer(nft.image_url).await.unwrap() }));

    let list_bytes = future::join_all(bytes_handles)
      .await
      .into_iter()
      .map(|h| h.unwrap())
      .collect::<Vec<ImageBuf>>();

    let mut image_block = ImageBuffer::new(360, 360);

    for (idx, buf) in list_bytes.into_iter().enumerate() {
      let square_size = idx + 1;

      let child_image_size = 360 / square_size;

      dbg!(child_image_size);

      let child_image = image::load_from_memory(&buf)?.resize(
        child_image_size as u32,
        child_image_size as u32,
        image::imageops::FilterType::Lanczos3,
      );

      let x = (idx % child_image_size) * child_image_size;
      let y = (idx / child_image_size) * child_image_size;

      image_block.copy_from(&child_image, x as u32, y as u32)?;
    }

    image_block.save(format!("block_{block}.png"))?;
  }

  Ok(())
}
