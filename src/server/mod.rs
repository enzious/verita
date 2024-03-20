use actix_web::dev::Server;
use actix_web::*;
use colored::Colorize;

use crate::apis;

pub mod config;

use self::config::FuzionVeritaConfig;

pub async fn build(config: &FuzionVeritaConfig) -> Result<Server, ()> {
  info!(
    "{} {}",
    "Listening locally at:".blue(),
    &config.http.get_uri(),
  );

  let db_pool = config.database.get_db_pool().await;

  let srv = {
    let config = config.to_owned();

    HttpServer::new(move || {
      let app = App::new();

      app
        .app_data(web::Data::new(config.to_owned()))
        .app_data(web::Data::new(db_pool.to_owned()))
        .configure(|config| apis::build(config))
    })
  }
  .shutdown_timeout(5u64)
  .bind(format!("{}:{}", &config.http.host, &config.http.port))
  .expect("Cannot bind.")
  .run();

  Ok(srv)
}
