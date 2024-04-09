use ts_rs::TS;

use crate::dao::realm::RealmId;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../web/src/js/dto/identity.ts")]
#[serde(rename_all = "camelCase")]
pub struct Identity {
  pub realm: String,
  pub realm_id: RealmId,
  pub username: String,
}
