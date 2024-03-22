use actix_web_thiserror::ResponseError;
use fuzion_commons::db::PgClientError;
use thiserror::Error;

pub struct CredentialService;

impl CredentialService {}

#[derive(Debug, Error, ResponseError)]
pub enum CredentialServiceError {
  #[error("internal error")]
  InternalError,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
}
