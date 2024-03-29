use actix_web_thiserror::ResponseError;
use fuzion_commons::db::{PgClient, PgClientError};
use thiserror::Error;

use crate::dao::realm::Realm;
use crate::repos::realm::RealmRepo;
use crate::repos::RepoError;

pub struct RealmService;

impl RealmService {
  pub async fn get_realm_by_name_required(
    db_client: &PgClient<'_>,
    realm: &str,
  ) -> Result<Realm, RealmServiceError> {
    RealmRepo::get_realm_by_name(&db_client, realm)
      .await?
      .ok_or(RealmServiceError::InvalidRealm)
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum RealmServiceError {
  #[error("internal error")]
  InternalError,
  #[error("invalid realm")]
  InvalidRealm,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
  #[error(transparent)]
  RealmError(#[from] RepoError),
}
