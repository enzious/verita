use chrono::prelude::*;

use super::credential::CredentialConfigId;

pub type UserId = i64;

pub struct User {
  pub id: Option<UserId>,
  pub realm_id: i32,
  pub username: String,
  pub email: Option<String>,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}

pub struct UserCredential {
  pub user_id: UserId,
  pub credential_config_id: CredentialConfigId,
  pub content: String,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}
