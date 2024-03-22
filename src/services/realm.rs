use actix_web_thiserror::ResponseError;
use fuzion_commons::db::PgClientError;
use thiserror::Error;

pub struct RealmService;

impl RealmService {}

#[derive(Debug, Error, ResponseError)]
pub enum RealmServiceError {
  #[error("internal error")]
  InternalError,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
}
