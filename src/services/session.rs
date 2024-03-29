use actix_web_thiserror::ResponseError;
use fuzion_commons::db::{PgClient, PgClientError};
use thiserror::Error;

use crate::dao::realm::RealmId;
use crate::repos::user::UserRepo;
use crate::repos::RepoError;

use super::user::{UserService, UserServiceError};

pub struct SessionService;

impl SessionService {
  pub async fn login(
    db_client: &PgClient<'_>,
    realm_id: RealmId,
    user: &str,
    password: &str,
  ) -> Result<(), SessionServiceError> {
    let user_id = UserRepo::get_user_by_username_email(&db_client, realm_id, &user)
      .await?
      .and_then(|user| user.id)
      .ok_or(SessionServiceError::InvalidUser)?;

    let user_credential = UserService::verify_credential(db_client, user_id, password).await?;

    Ok(())
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum SessionServiceError {
  #[error("internal error")]
  InternalError,
  #[error("no such user")]
  InvalidUser,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
  #[error(transparent)]
  RepoError(#[from] RepoError),
  #[error(transparent)]
  UserServiceError(#[from] UserServiceError),
}
