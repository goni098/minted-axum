use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "categories")]
pub enum Categories {
  #[sea_orm(string_value = "NFT")]
  NFT,

  #[sea_orm(string_value = "BAYC")]
  BAYC,

  #[sea_orm(string_value = "CRYPTOPUNKS")]
  CRYPTOPUNKS,

  #[sea_orm(string_value = "ETH")]
  ETH,

  #[sea_orm(string_value = "SOL")]
  SOL,

  #[sea_orm(string_value = "ENS")]
  ENS,

  #[sea_orm(string_value = "DEGEN")]
  DEGEN,

  #[sea_orm(string_value = "BLUECHIP")]
  BLUECHIP,

  #[sea_orm(string_value = "GIF")]
  GIF,

  #[sea_orm(string_value = "LAND")]
  LAND,

  #[sea_orm(string_value = "SOUND")]
  SOUND,

  #[sea_orm(string_value = "PUNK")]
  PUNK,

  #[sea_orm(string_value = "APE")]
  APE,

  #[sea_orm(string_value = "AZUKI")]
  AZUKI,

  #[sea_orm(string_value = "DOODLES")]
  DOODLES,

  #[sea_orm(string_value = "COOLCATS")]
  COOLCATS,

  #[sea_orm(string_value = "BEEPLE")]
  BEEPLE,

  #[sea_orm(string_value = "DEGODS")]
  DEGODS,

  #[sea_orm(string_value = "PEPE")]
  PEPE,

  #[sea_orm(string_value = "DECENTRALAND")]
  DECENTRALAND,

  #[sea_orm(string_value = "SANDBOX")]
  SANDBOX,

  #[sea_orm(string_value = "CLONEX")]
  CLONEX,

  #[sea_orm(string_value = "CYBERKONGZ")]
  CYBERKONGZ,

  #[sea_orm(string_value = "MOONBIRDS")]
  MOONBIRDS,

  #[sea_orm(string_value = "MEEBITS")]
  MEEBITS,

  #[sea_orm(string_value = "VEEFRIENDS")]
  VEEFRIENDS,

  #[sea_orm(string_value = "WORLDOFWOMEN")]
  WORLDOFWOMEN,

  #[sea_orm(string_value = "GOBLINTOWN")]
  GOBLINTOWN,

  #[sea_orm(string_value = "PUDGYPENGUINS")]
  PUDGYPENGUINS,

  #[sea_orm(string_value = "CRYPTOADZ")]
  CRYPTOADZ,

  #[sea_orm(string_value = "INVISIBLEFRIENDS")]
  INVISIBLEFRIENDS,

  #[sea_orm(string_value = "GUTTERGANG")]
  GUTTERGANG,

  #[sea_orm(string_value = "MFERS")]
  MFERS,

  #[sea_orm(string_value = "DEADFELLAZ")]
  DEADFELLAZ,

  #[sea_orm(string_value = "DIGIDAIGAKU")]
  DIGIDAIGAKU,

  #[sea_orm(string_value = "NOUNS")]
  NOUNS,

  #[sea_orm(string_value = "CRYPTODICKBUTTS")]
  CRYPTODICKBUTTS,

  #[sea_orm(string_value = "FREE")]
  FREE,

  #[sea_orm(string_value = "LOWCOST")]
  LOWCOST,

  #[sea_orm(string_value = "VERIFIED")]
  VERIFIED,

  #[sea_orm(string_value = "LOOKSRARE")]
  LOOKSRARE,

  #[sea_orm(string_value = "OPENSEA")]
  OPENSEA,

  #[sea_orm(string_value = "MAGICEDEN")]
  MAGICEDEN,
}
