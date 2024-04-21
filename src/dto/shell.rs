use ts_rs::TS;

use crate::dao::realm::Realm;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../web/src/js/dto/shell.ts")]
pub struct ShellInit {
  pub realms: Vec<Realm>,
}
