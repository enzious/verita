use fuzion_commons::migration::Migrator;
use fuzion_commons::migration::BASE_MODULE_NAME;

use crate::server::config::FuzionVeritaConfig;

pub async fn init(config: &FuzionVeritaConfig, client: deadpool_postgres::Client) {
  if config.migrate {
    let mut migrator = Migrator::new(BASE_MODULE_NAME, client, vec![]);

    if let Err(err) = migrator.migrate().await {
      panic!("Failed to migrate database: {}", &err);
    }
  }
}
