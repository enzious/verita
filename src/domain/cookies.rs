use std::collections::HashMap;
use std::future::{self, Future};

use actix_web::cookie::Cookie;
use actix_web::web;
use actix_web::{FromRequest, HttpRequest};
use actix_web_thiserror::ResponseError;
use hmac::digest::InvalidLength;
use thiserror::Error;

use crate::dao::realm::RealmId;

use super::identity::Identity;
use super::jwt::{VeritaJwt, VeritaJwtError, VeritaJwtKey};

pub const VERITA_IDENTITY: &'static str = "VERITA_IDENTITY";

#[derive(Debug, Default)]
pub struct SessionCookies {
  identity: HashMap<RealmId, Identity>,
}

impl SessionCookies {
  pub fn from_request<'c>(
    request: &HttpRequest,
    VeritaJwtKey(key): &VeritaJwtKey,
  ) -> Result<SessionCookies, SessionCookieError> {
    let mut identity: Vec<Identity> = match request.cookie(VERITA_IDENTITY) {
      Some(cookie) => VeritaJwt::<Vec<Identity>>::from_string(cookie.value(), key.as_bytes())
        .map(|jwt| jwt.into_data())
        .unwrap_or_else(|err| {
          error!("Failed to decode cookie: {:?}", &err);

          vec![]
        }),
      _ => vec![],
    };

    Ok(SessionCookies {
      identity: identity
        .drain(..)
        .map(|identity| (identity.realm, identity))
        .collect(),
    })
  }

  pub fn insert_identity(&mut self, identity: Identity) -> Option<Identity> {
    self.identity.insert(identity.realm, identity)
  }

  pub fn remove_identity(&mut self, realm_id: RealmId) -> Option<Identity> {
    self.identity.remove(&realm_id)
  }

  pub fn to_cookie<'c>(
    &self,
    VeritaJwtKey(key): &VeritaJwtKey,
  ) -> Result<Cookie<'c>, SessionCookieError> {
    Ok(Cookie::new(
      VERITA_IDENTITY,
      VeritaJwt::new(self.identity.to_owned().values().collect::<Vec<_>>())
        .to_string(key.as_bytes())?,
    ))
  }

  pub fn get_identity(&self, realm_id: RealmId) -> Option<&Identity> {
    self.identity.get(&realm_id)
  }

  pub fn get_active_identity(&self, realm_id: RealmId) -> Option<&Identity> {
    self.identity.get(&realm_id)
  }
}

impl FromRequest for SessionCookies {
  type Error = SessionCookieError;
  type Future = Box<dyn Future<Output = Result<Self, Self::Error>> + Unpin>;

  fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
    if let Some(key) = req.app_data::<web::Data<VeritaJwtKey>>() {
      return Box::new(future::ready(SessionCookies::from_request(&req, &key)));
    }

    Box::new(future::ready(Ok(Default::default())))
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
