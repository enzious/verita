use actix_web_thiserror::ResponseError;
use fuzion_commons::db::PgClientError;
use thiserror::Error;

pub mod credential;
pub mod realm;
pub mod session;
pub mod user;

#[derive(Debug, Error, ResponseError)]
pub enum RepoError {
  #[error("internal error")]
  InternalError,
  #[error("no record")]
  NoRecord,
  #[error(transparent)]
  Postgres(#[from] postgres::Error),
  #[error(transparent)]
  PostgreClient(#[from] PgClientError),
  #[error(transparent)]
  SerdeJsonError(#[from] serde_json::Error),
}
