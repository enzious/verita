use chrono::prelude::*;
use postgres::Row;
use smart_default::SmartDefault;

use super::{credential::CredentialConfigId, realm::RealmId};

pub type UserId = i64;

#[derive(SmartDefault)]
pub struct User {
  pub id: Option<UserId>,
  pub realm_id: i32,
  pub username: String,
  pub email: Option<String>,
  pub email_verified: bool,
  pub operator: bool,
  #[default(_code = "Utc::now()")]
  pub created: DateTime<Utc>,
  #[default(_code = "Utc::now()")]
  pub updated: DateTime<Utc>,
}

impl From<&Row> for User {
  fn from(row: &Row) -> User {
    User {
      id: Some(row.get::<_, UserId>("id")),
      realm_id: row.get::<_, RealmId>("realm_id"),
      username: row.get::<_, String>("username"),
      email: row.get::<_, Option<String>>("email"),
      email_verified: row.get::<_, bool>("email_verified"),
      operator: row.get::<_, bool>("operator"),
      created: row.get::<_, DateTime<Utc>>("created"),
      updated: row.get::<_, DateTime<Utc>>("updated"),
    }
  }
}

pub struct UserCredential {
  pub user_id: UserId,
  pub credential_config_id: CredentialConfigId,
  pub content: String,
  pub temporary: bool,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}

impl From<&Row> for UserCredential {
  fn from(row: &Row) -> UserCredential {
    UserCredential {
      user_id: row.get::<_, UserId>("user_id"),
      credential_config_id: row.get::<_, CredentialConfigId>("credential_config_id"),
      content: row.get::<_, String>("content"),
      temporary: row.get::<_, bool>("temporary"),
      created: row.get::<_, DateTime<Utc>>("created"),
      updated: row.get::<_, DateTime<Utc>>("updated"),
    }
  }
}
