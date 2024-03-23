use actix_web_thiserror::ResponseError;
use fuzion_commons::db::{PgClient, PgClientError};
use thiserror::Error;

use crate::{
  dao::{realm::Realm, user::User},
  repos::{realm::RealmRepo, user::UserRepo, RepoError},
  server::config::FuzionVeritaConfig,
};

pub struct SetupService;

impl SetupService {
  pub async fn init(config: &FuzionVeritaConfig) -> Result<(), SetupServiceError> {
    let db_pool = config
      .database
      .get_db_pool()
      .await
      .map_err(|_| SetupServiceError::InternalError)?;

    let mut db_client: PgClient<'_> = db_pool
      .get()
      .await
      .map_err(|_| SetupServiceError::InternalError)?
      .into();

    let txn = db_client.transaction().await?;

    let operator_realm = match RealmRepo::get_operator_realm(&txn).await? {
      Some(realm) => realm,
      None => {
        RealmRepo::insert_realm(
          &txn,
          &Realm {
            name: String::from("verita"),
            operator: true,
            ..Default::default()
          },
        )
        .await?
      }
    };

    let _operator_user = match (operator_realm.id, UserRepo::get_operator_user(&txn).await?) {
      (_, Some(user)) => user,
      (Some(realm_id), None) => {
        UserRepo::insert_user(
          &txn,
          &User {
            realm_id,
            username: String::from("verita"),
            operator: true,
            ..Default::default()
          },
        )
        .await?
      }
      _ => Err(SetupServiceError::InternalError)?,
    };

    txn.commit().await?;

    Ok(())
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum SetupServiceError {
  #[error("internal error")]
  InternalError,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
  #[error(transparent)]
  RepoError(#[from] RepoError),
}
