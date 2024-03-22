use chrono::prelude::*;
use postgres::Row;
use smart_default::SmartDefault;

pub type RealmId = i32;

#[derive(SmartDefault)]
pub struct Realm {
  pub id: Option<RealmId>,
  pub name: String,
  pub operator: bool,
  #[default(_code = "Utc::now()")]
  pub created: DateTime<Utc>,
  #[default(_code = "Utc::now()")]
  pub updated: DateTime<Utc>,
}

impl From<&Row> for Realm {
  fn from(row: &Row) -> Realm {
    Realm {
      id: Some(row.get::<_, RealmId>("id")),
      name: row.get::<_, String>("name"),
      operator: row.get::<_, bool>("operator"),
      created: row.get::<_, DateTime<Utc>>("created"),
      updated: row.get::<_, DateTime<Utc>>("updated"),
    }
  }
}
