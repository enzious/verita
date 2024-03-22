use chrono::prelude::*;

use super::realm::RealmId;

pub type CredentialConfigId = i32;

pub struct CredentialConfig {
  pub id: Option<CredentialConfigId>,
  pub realm_id: RealmId,
  pub hash: String,
  pub iterations: i32,
  pub salt: Option<String>,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}
