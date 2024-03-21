use chrono::prelude::*;

pub type CredentialConfigId = i32;

pub struct CredentialConfig {
  pub id: Option<CredentialConfigId>,
  pub hash: String,
  pub salt: Option<String>,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}
