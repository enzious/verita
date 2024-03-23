use actix_web_thiserror::ResponseError;
use fuzion_commons::db::PgClientError;
use thiserror::Error;

use crate::{dao::credential::CredentialConfig, repos::RepoError};

pub struct CredentialService;

impl CredentialService {
  pub fn hash_credential(
    _credential_config: &CredentialConfig,
    _credential: &str,
  ) -> Result<(), RepoError> {
    Ok(())
  }

  pub fn generate_salt(credential_config: &CredentialConfig, username: &str) -> Vec<u8> {
    let mut salt_len = username.as_bytes().len();

    if let Some(ref config_salt) = credential_config.salt {
      salt_len += config_salt.len();
    }

    let mut salt = Vec::with_capacity(salt_len);

    if let Some(ref config_salt) = credential_config.salt {
      salt.extend(config_salt);
    }

    salt.extend(username.as_bytes());

    salt
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum CredentialServiceError {
  #[error("internal error")]
  InternalError,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
}
