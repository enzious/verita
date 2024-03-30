use actix_web_thiserror::ResponseError;
use fuzion_commons::db::{PgClient, PgClientError};
use thiserror::Error;

use crate::{
  dao::{
    credential::CredentialConfig,
    realm::{Realm, RealmId},
    user::{User, UserCredential, UserId},
  },
  repos::{credential::CredentialRepo, realm::RealmRepo, user::UserRepo, RepoError},
  server::config::FuzionVeritaConfig,
};

use super::{
  credential::CredentialServiceError,
  user::{UserService, UserServiceError},
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

    // Set up the operator realm.
    let realm = Self::init_operator_realm(&txn).await?;
    let realm_id = realm.id.ok_or(SetupServiceError::InternalError)?;

    // Set up the operator realm credential config.
    let credential_config = Self::init_operator_realm_credential_config(&txn, realm_id).await?;

    // Set up the operator user.
    let user = Self::init_operator_user(
      &txn,
      realm_id,
      config.admin.as_ref().map(|username| username as &str),
    )
    .await?;
    let user_id = user.id.ok_or(SetupServiceError::InternalError)?;

    // Set up the operator user credential.
    if let Some(user_password) = &config.admin_password {
      let _ =
        Self::init_operator_user_credential(&txn, &credential_config, user_id, &user_password)
          .await?;
    }

    txn.commit().await?;

    Ok(())
  }

  pub async fn init_operator_realm(db_client: &PgClient<'_>) -> Result<Realm, SetupServiceError> {
    match RealmRepo::get_operator_realm(&db_client).await? {
      Some(realm) => Ok(realm),
      None => Ok(
        RealmRepo::insert_realm(
          &db_client,
          &Realm {
            name: String::from("verita"),
            operator: true,
            ..Default::default()
          },
        )
        .await?,
      ),
    }
  }

  pub async fn init_operator_realm_credential_config(
    db_client: &PgClient<'_>,
    realm_id: RealmId,
  ) -> Result<CredentialConfig, SetupServiceError> {
    match CredentialRepo::get_credential_config_by_realm_id(db_client, realm_id).await? {
      Some(credential_config) => Ok(credential_config),
      None => Ok(
        CredentialRepo::insert_credential_config(
          db_client,
          &CredentialConfig {
            realm_id,
            ..Default::default()
          },
        )
        .await?,
      ),
    }
  }

  pub async fn init_operator_user(
    db_client: &PgClient<'_>,
    realm_id: RealmId,
    username: Option<&str>,
  ) -> Result<User, SetupServiceError> {
    match UserRepo::get_operator_user(&db_client).await? {
      Some(mut user) => {
        if let Some(username) = username {
          if username != user.username {
            if UserRepo::get_user_by_username(&db_client, realm_id, username)
              .await?
              .is_some()
            {
              return Err(SetupServiceError::AdminUsernameTaken);
            }

            user.username = username.to_string();
            return Ok(UserRepo::update_user(&db_client, &user).await?);
          }
        }

        Ok(user)
      }
      None => Ok(
        UserRepo::insert_user(
          &db_client,
          &User {
            realm_id,
            username: username
              .map(|username| username.to_string())
              .unwrap_or(String::from("verita")),
            operator: true,
            ..Default::default()
          },
        )
        .await?,
      ),
    }
  }

  pub async fn init_operator_user_credential(
    db_client: &PgClient<'_>,
    _config: &CredentialConfig,
    user_id: UserId,
    credential: &str,
  ) -> Result<UserCredential, SetupServiceError> {
    let user_credential = UserService::verify_credential(db_client, user_id, credential).await;

    match user_credential {
      Err(UserServiceError::NoUserCredential)
      | Err(UserServiceError::CredentialServiceError(
        CredentialServiceError::RingUnspecifiedError(_),
      )) => Ok(UserService::update_credential(db_client, user_id, credential).await?),
      Ok(credential) => Ok(credential),
      _ => Err(SetupServiceError::InternalError),
    }
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum SetupServiceError {
  #[error("admin_username_taken")]
  AdminUsernameTaken,
  #[error("internal_error")]
  InternalError,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
  #[error(transparent)]
  RepoError(#[from] RepoError),
  #[error(transparent)]
  UserServiceError(#[from] UserServiceError),
}
