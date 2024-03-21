use chrono::prelude::*;

pub type RealmId = i32;

pub struct Realm {
  pub id: Option<RealmId>,
  pub name: String,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}
