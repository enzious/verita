use std::string::FromUtf16Error;

use actix_web_thiserror::ResponseError;
use base64::prelude::*;
use hmac::digest::InvalidLength;
use hmac::{Hmac, Mac};
use iter_tools::Itertools;
use sha2::Sha384;
use thiserror::Error;

pub type HmacSha384 = Hmac<Sha384>;

#[derive(Clone, Debug)]
pub struct VeritaJwtKey(pub(super) String);

impl VeritaJwtKey {
  pub fn new(key: impl Into<String>) -> Self {
    Self(key.into())
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VeritaJwtAlg {
  LZHS384,
  HS384,
}

impl Default for VeritaJwtAlg {
  fn default() -> Self {
    VeritaJwtAlg::HS384
  }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VeritaJwtHeader {
  alg: VeritaJwtAlg,
  typ: String,
}

#[derive(Debug)]
pub struct VeritaJwt<D> {
  header: VeritaJwtHeader,
  data: D,
}

impl<D> VeritaJwt<D> {
  pub fn new(data: D) -> Self {
    Self {
      header: Default::default(),
      data,
    }
  }

  pub fn to_string<'a>(&'a self, key: &[u8]) -> Result<String, VeritaJwtError>
  where
    D: serde::ser::Serialize,
  {
    let parts = vec![
      BASE64_STANDARD.encode(serde_json::to_string(&self.header)?),
      match self.header.alg {
        VeritaJwtAlg::HS384 => BASE64_STANDARD.encode(serde_json::to_string(&self.data)?),
        VeritaJwtAlg::LZHS384 => {
          lz_str::compress_to_base64(serde_json::to_string(&self.data)?.as_str())
        }
      },
    ]
    .into_iter()
    .join(".");

    let mac = match self.header.alg {
      VeritaJwtAlg::HS384 | VeritaJwtAlg::LZHS384 => {
        let mut mac = HmacSha384::new_from_slice(key)?;
        mac.update(parts.as_bytes());
        mac.finalize().into_bytes()
      }
    };

    let mac = BASE64_STANDARD.encode(mac);

    Ok(format!("{}.{}", &parts, &mac))
  }

  pub fn from_string(str: &str, key: &[u8]) -> Result<VeritaJwt<D>, VeritaJwtError>
  where
    D: for<'b> serde::de::Deserialize<'b>,
  {
    let mut parts = str.split(".");

    let (raw_header, data, signature) = match (parts.nth(0), parts.nth(0), parts.nth(0)) {
      (Some(header), Some(data), Some(signature)) => (header, data, signature),
      _ => return Err(VeritaJwtError::MissingPart),
    };

    let header: VeritaJwtHeader = serde_json::from_slice(&BASE64_STANDARD.decode(raw_header)?)?;

    match header.alg {
      VeritaJwtAlg::HS384 | VeritaJwtAlg::LZHS384 => {
        let mut mac = HmacSha384::new_from_slice(key)?;
        mac.update(format!("{}.{}", &raw_header, &data).as_bytes());

        if mac.finalize().into_bytes()[..] != BASE64_STANDARD.decode(&signature)?[..] {
          return Err(VeritaJwtError::InvalidSignature);
        }
      }
    };

    let data = match header.alg {
      VeritaJwtAlg::HS384 => serde_json::from_slice(&BASE64_STANDARD.decode(data)?)?,
      VeritaJwtAlg::LZHS384 => serde_json::from_str(&String::from_utf16(
        &lz_str::decompress_from_base64(data).ok_or(VeritaJwtError::LzDecompressError)?,
      )?)?,
    };

    Ok(VeritaJwt { header, data })
  }

  pub fn into_data(self) -> D {
    self.data
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum VeritaJwtError {
  #[error(transparent)]
  Base64Error(#[from] base64::DecodeError),
  #[error(transparent)]
  FromUtf16Error(#[from] FromUtf16Error),
  #[error(transparent)]
  InvalidKeyLength(#[from] InvalidLength),
  #[error("invalid_signature")]
  InvalidSignature,
  #[error("lz_decompress_error")]
  LzDecompressError,
  #[error("missing_jwt_part")]
  MissingPart,
  #[error(transparent)]
  SerdeError(#[from] serde_json::Error),
}

#[cfg(test)]
mod test {
  use serde_json::json;

  use super::VeritaJwt;

  #[test]
  pub fn test() {
    let jwt = VeritaJwt::new(json! {{
        "realm": 0,
        "user": 0,
        "session": "",
    }});

    println!("base: {:?}", &jwt);
    println!("destination: {:?}", &jwt.to_string(b""));
    println!(
      "return: {:?}",
      &jwt
        .to_string(b"")
        .map(|jwt| VeritaJwt::<serde_json::Value>::from_string(&jwt, b""))
    );
  }
}
