use actix_web::dev::Server;
use actix_web::*;
use colored::Colorize;
use fuzion_commons::config::DatabaseConfigError;
use thiserror::Error;

use crate::apis;
use crate::domain::jwt::VeritaJwtKey;

pub mod config;

use self::config::FuzionVeritaConfig;

pub async fn build(config: &FuzionVeritaConfig) -> Result<Server, ServerError> {
  info!(
    "{} {}",
    "Listening locally at:".blue(),
    &config.http.get_uri(),
  );

  let db_pool = config.database.get_db_pool().await?;

  let jwt_key = VeritaJwtKey::new("blah");

  let srv = {
    let config = config.to_owned();

    HttpServer::new(move || {
      let app = App::new();

      app
        .app_data(web::Data::new(config.to_owned()))
        .app_data(web::Data::new(db_pool.to_owned()))
        .app_data(web::Data::new(jwt_key.to_owned()))
        .configure(|config| apis::build(config))
    })
  }
  .shutdown_timeout(5u64)
  .bind(format!("{}:{}", &config.http.host, &config.http.port))
  .expect("Cannot bind.")
  .run();

  Ok(srv)
}

#[derive(Debug, Error)]
pub enum ServerError {
  #[error(transparent)]
  DatabaseConfigError(#[from] DatabaseConfigError),
}
