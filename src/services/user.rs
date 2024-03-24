use actix_web_thiserror::ResponseError;
use fuzion_commons::db::{PgClient, PgClientError};
use thiserror::Error;

use crate::{
  dao::user::{UserCredential, UserId},
  repos::{credential::CredentialRepo, user::UserRepo, RepoError},
};

use super::credential::{CredentialService, CredentialServiceError};

pub struct UserService;

impl UserService {
  pub async fn verify_credential(
    db_client: &PgClient<'_>,
    user_id: UserId,
    credential: &str,
  ) -> Result<UserCredential, UserServiceError> {
    let user = UserRepo::get_user_by_id(db_client, user_id)
      .await?
      .ok_or(UserServiceError::InvalidUser)?;
    let user_credential = UserRepo::get_user_credential_by_user_id(db_client, user_id)
      .await?
      .ok_or(UserServiceError::NoUserCredential)?;
    let credential_config =
      CredentialRepo::get_credential_config_by_realm_id(db_client, user.realm_id)
        .await?
        .ok_or(UserServiceError::InternalError)?;

    CredentialService::verify_credential(
      &credential_config,
      &user_credential.content,
      &user.username,
      credential,
    )?;

    Ok(user_credential)
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum UserServiceError {
  #[error(transparent)]
  CredentialServiceError(#[from] CredentialServiceError),
  #[error("internal error")]
  InternalError,
  #[error("no user")]
  InvalidUser,
  #[error("no user credential")]
  NoUserCredential,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
  #[error(transparent)]
  RepoError(#[from] RepoError),
}
