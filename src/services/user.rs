use actix_web_thiserror::ResponseError;
use fuzion_commons::db::{PgClient, PgClientError};
use thiserror::Error;

use crate::{
  dao::{
    credential::CredentialConfig,
    user::{UserCredential, UserId},
  },
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
    let user = UserRepo::get_user(db_client, user_id)
      .await?
      .ok_or(UserServiceError::InvalidUser)?;
    let user_credential = UserRepo::get_user_credential_by_user_id(db_client, user_id)
      .await?
      .ok_or(UserServiceError::NoUserCredential)?;
    let credential_config: CredentialConfig =
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

  pub async fn update_credential(
    db_client: &PgClient<'_>,
    user_id: UserId,
    credential: &str,
  ) -> Result<UserCredential, UserServiceError> {
    let user = UserRepo::get_user(db_client, user_id)
      .await?
      .ok_or(UserServiceError::InvalidUser)?;
    let credential_config =
      CredentialRepo::get_credential_config_by_realm_id(db_client, user.realm_id)
        .await?
        .ok_or(UserServiceError::InternalError)?;
    let credential_config_id = credential_config
      .id
      .ok_or(UserServiceError::InternalError)?;

    UserRepo::delete_user_credential(db_client, user_id).await?;

    let content =
      CredentialService::hash_credential(&credential_config, &user.username, credential)?;

    Ok(
      UserRepo::insert_user_credential(
        db_client,
        &UserCredential {
          user_id,
          credential_config_id,
          content,
          ..Default::default()
        },
      )
      .await?,
    )
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum UserServiceError {
  #[error(transparent)]
  CredentialServiceError(#[from] CredentialServiceError),
  #[error("internal_error")]
  InternalError,
  #[error("no_user")]
  InvalidUser,
  #[error("no_user_credential")]
  NoUserCredential,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
  #[error(transparent)]
  RepoError(#[from] RepoError),
}
