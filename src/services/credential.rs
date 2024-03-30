use std::num::NonZeroU32;

use actix_web_thiserror::ResponseError;
use fuzion_commons::db::PgClientError;
use rand::Rng;
use ring::{digest, error::Unspecified, pbkdf2};
use thiserror::Error;

use crate::dao::credential::CredentialConfig;

pub struct CredentialService;

impl CredentialService {
  pub fn hash_credential(
    config: &CredentialConfig,
    username: &str,
    credential: &str,
  ) -> Result<Vec<u8>, CredentialServiceError> {
    let salt = Self::compile_salt(&config, username);

    let mut out: [u8; digest::SHA256_OUTPUT_LEN] = [0u8; digest::SHA256_OUTPUT_LEN];

    pbkdf2::derive(
      pbkdf2::PBKDF2_HMAC_SHA256,
      NonZeroU32::new(config.iterations as u32).ok_or(CredentialServiceError::InternalError)?,
      &salt,
      credential.as_bytes(),
      &mut out,
    );

    Ok(out.to_vec())
  }

  pub fn verify_credential(
    config: &CredentialConfig,
    hash: &Vec<u8>,
    username: &str,
    credential: &str,
  ) -> Result<(), CredentialServiceError> {
    let salt = Self::compile_salt(&config, username);

    pbkdf2::verify(
      pbkdf2::PBKDF2_HMAC_SHA256,
      NonZeroU32::new(config.iterations as u32).ok_or(CredentialServiceError::InternalError)?,
      &salt,
      credential.as_bytes(),
      &hash[..],
    )?;

    Ok(())
  }

  pub fn compile_salt(credential_config: &CredentialConfig, username: &str) -> Vec<u8> {
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

  pub fn generate_salt(len: usize) -> Vec<u8> {
    let mut salt = vec![0; len];

    let mut rng = rand::thread_rng();
    rng.fill(&mut salt[..]);

    salt
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum CredentialServiceError {
  #[error("internal_error")]
  InternalError,
  #[error(transparent)]
  PostgresError(#[from] PgClientError),
  #[error("unspecified_ring_error")]
  RingUnspecifiedError(#[from] Unspecified),
}

#[cfg(test)]
mod test {
  use super::CredentialService;

  #[test]
  fn generate_salt() {
    let len = 16;
    let result = CredentialService::generate_salt(len);

    assert_eq!(result.len(), len);
  }
}
