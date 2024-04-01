use std::collections::HashMap;
use std::future::{self, Future};

use actix_web::cookie::Cookie;
use actix_web::{FromRequest, HttpRequest};
use actix_web_thiserror::ResponseError;
use hmac::digest::InvalidLength;
use thiserror::Error;

use crate::dao::realm::RealmId;
use crate::dao::user::UserId;

use super::jwt::{VeritaJwt, VeritaJwtError};

pub const VERITA_IDENTITY: &'static str = "VERITA_IDENTITY";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identity {
  realm: RealmId,
  subject: UserId,
  session: String,
}

pub struct SessionCookies {
  identity: HashMap<RealmId, Identity>,
}

impl SessionCookies {
  pub fn from_request<'c>(request: &HttpRequest) -> Result<SessionCookies, SessionCookieError> {
    let mut identity: Vec<Identity> = match request.cookie("identity") {
      Some(cookie) => VeritaJwt::<Vec<Identity>>::try_from(cookie.value())
        .map(|jwt| jwt.into_data())
        .unwrap_or(vec![]),
      _ => vec![],
    };

    Ok(SessionCookies {
      identity: identity
        .drain(..)
        .map(|identity| (identity.realm, identity))
        .collect(),
    })
  }

  pub fn to_cookies<'c>(&self) -> Result<Vec<Cookie<'c>>, SessionCookieError> {
    Ok(vec![Cookie::new(
      VERITA_IDENTITY,
      VeritaJwt::new(self.identity.to_owned()).to_string()?,
    )])
  }

  pub fn get_identity(&self, realm_id: RealmId) -> Option<&Identity> {
    self.identity.get(&realm_id)
  }
}

impl FromRequest for SessionCookies {
  type Error = SessionCookieError;
  type Future = Box<dyn Future<Output = Result<Self, Self::Error>> + Unpin>;

  fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
    Box::new(future::ready(SessionCookies::from_request(&req)))
  }
}

#[derive(Debug, Error, ResponseError)]
pub enum SessionCookieError {
  #[error(transparent)]
  Base64Error(#[from] base64::DecodeError),
  #[error(transparent)]
  SerdeError(#[from] serde_json::Error),
  #[error(transparent)]
  InvalidKeyLength(#[from] InvalidLength),
  #[error("missing_jwt_part")]
  MissingPart,
  #[error(transparent)]
  VeritaJwtError(#[from] VeritaJwtError),
}
