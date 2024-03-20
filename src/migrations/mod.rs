use fuzion_commons::migration::Migrator;
use fuzion_commons::migration::BASE_MODULE_NAME;

use crate::server::config::FuzionVeritaConfig;

pub async fn init(config: &FuzionVeritaConfig) {
  if config.migrate {
    let db_pool = config
      .database
      .get_db_pool()
      .await
      .expect("Failed to initialize DB pool.");

    let db_conn = db_pool.get().await.expect("");
    let mut migrator = Migrator::new(BASE_MODULE_NAME, db_conn, vec![]);

    if let Err(err) = migrator.migrate().await {
      panic!("Failed to migrate database: {}", &err);
    }
  }
}
