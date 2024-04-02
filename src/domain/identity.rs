use crate::dao::realm::RealmId;
use crate::dao::user::UserId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identity {
  pub realm: RealmId,
  pub subject: UserId,
  pub session: String,
}

impl Identity {
  pub fn new(realm: RealmId, subject: UserId, session: impl Into<String>) -> Self {
    Self {
      realm,
      subject,
      session: session.into(),
    }
  }
}
