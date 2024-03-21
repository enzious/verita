use actix_web_thiserror::ResponseError;
use fuzion_commons::db::PgClientError;
use thiserror::Error;

pub struct SessionService;

impl SessionService {}

#[derive(Debug, Error, ResponseError)]
pub enum UserServiceError {
  #[error("internal error")]
  InternalError,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
}
