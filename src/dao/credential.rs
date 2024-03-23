use chrono::prelude::*;
use postgres::Row;
use smart_default::SmartDefault;

use super::realm::RealmId;

pub type CredentialConfigId = i32;

#[derive(Clone, Debug, SmartDefault)]
pub struct CredentialConfig {
  pub id: Option<CredentialConfigId>,
  pub realm_id: RealmId,
  pub hash: Vec<u8>,
  pub iterations: i32,
  pub salt: Option<Vec<u8>>,
  #[default(_code = "Utc::now()")]
  pub created: DateTime<Utc>,
  #[default(_code = "Utc::now()")]
  pub updated: DateTime<Utc>,
}

impl From<&Row> for CredentialConfig {
  fn from(row: &Row) -> CredentialConfig {
    CredentialConfig {
      id: Some(row.get::<_, CredentialConfigId>("id")),
      realm_id: row.get::<_, RealmId>("realm_id"),
      hash: row.get::<_, Vec<u8>>("hash"),
      iterations: row.get::<_, i32>("iterations"),
      salt: row.get::<_, Option<Vec<u8>>>("salt"),
      created: row.get::<_, DateTime<Utc>>("created"),
      updated: row.get::<_, DateTime<Utc>>("updated"),
    }
  }
}
