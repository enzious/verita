use actix_files::Files;
use actix_web::*;
use actix_web_thiserror::{set_global_transform, ResponseTransform};
use serde_json::json;

mod hello;
mod session;
mod views;

pub struct ErrorResponse;

impl ResponseTransform for ErrorResponse {
  fn transform(
    &self,
    _name: &str,
    _err: &dyn std::error::Error,
    status_code: actix_web::http::StatusCode,
    reason: Option<serde_json::Value>,
  ) -> HttpResponse {
    actix_web::HttpResponse::build(status_code).json(json! {{
      "error": reason,
    }})
  }

  fn default_error_status_code(&self) -> actix_web::http::StatusCode {
    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
  }
}

pub fn build(config: &mut web::ServiceConfig) {
  set_global_transform(ErrorResponse {});

  config
    .service(
      Scope::new("/api")
        .service(session::build())
        .service(views::build())
        .service(hello::hello),
    )
    .service(Files::new("/", "./static/").prefer_utf8(true));
}
