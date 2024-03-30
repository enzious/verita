use crate::dao::realm::RealmId;

#[derive(Debug, Deserialize)]
pub struct LoginInitRequest {
  pub realm: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
  pub realm_id: RealmId,
  pub user: String,
  pub password: String,
}
