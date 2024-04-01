use actix_web_thiserror::ResponseError;
use base64::prelude::*;
use hmac::digest::InvalidLength;
use hmac::{Hmac, Mac};
use iter_tools::Itertools;
use sha2::Sha384;
use thiserror::Error;

pub type HmacSha384 = Hmac<Sha384>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JwtHeader {
  alg: String,
  typ: String,
}

pub struct VeritaJwt<D> {
  header: JwtHeader,
  data: D,
}

impl<D> VeritaJwt<D> {
  pub fn new(data: D) -> Self {
    Self {
      header: Default::default(),
      data,
    }
  }

  pub fn to_string<'a>(&'a self) -> Result<String, VeritaJwtError>
  where
    D: serde::ser::Serialize,
  {
    let parts = vec![
      BASE64_STANDARD.encode(serde_json::to_string(&self.header)?),
      BASE64_STANDARD.encode(serde_json::to_string(&self.data)?),
    ]
    .into_iter()
    .join(".");

    let mut mac = HmacSha384::new_from_slice(b"")?;
    mac.update(parts.as_bytes());

    let mac = BASE64_STANDARD.encode(mac.finalize().into_bytes());

    Ok(format!("{}.{}", &parts, &mac))
  }

  pub fn into_data(self) -> D {
    self.data
  }
}

impl<D> TryFrom<&str> for VeritaJwt<D>
where
  D: for<'b> serde::de::Deserialize<'b>,
{
  type Error = VeritaJwtError;

  fn try_from(str: &str) -> Result<Self, Self::Error> {
    let mut parts = str.split(".");

    let (header, data, signature) = match (parts.nth(0), parts.nth(1), parts.nth(2)) {
      (Some(header), Some(data), Some(signature)) => (header, data, signature),
      _ => return Err(VeritaJwtError::MissingPart),
    };

    let mut mac = HmacSha384::new_from_slice(b"")?;
    mac.update(format!("{}.{}", &header, &data).as_bytes());

    if mac.finalize().into_bytes()[..] != BASE64_STANDARD.decode(&signature)?[..] {
      return Err(VeritaJwtError::InvalidSignature);
    }

    let header = serde_json::from_slice(&BASE64_STANDARD.decode(header)?)?;
    let data = serde_json::from_slice(&BASE64_STANDARD.decode(data)?)?;

    Ok(VeritaJwt { header, data })
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum VeritaJwtError {
  #[error(transparent)]
  Base64Error(#[from] base64::DecodeError),
  #[error(transparent)]
  SerdeError(#[from] serde_json::Error),
  #[error(transparent)]
  InvalidKeyLength(#[from] InvalidLength),
  #[error("invalid_signature")]
  InvalidSignature,
  #[error("missing_jwt_part")]
  MissingPart,
}
