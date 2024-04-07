use actix_web::*;

use crate::dao::realm::RealmId;
use crate::domain::cookies::SessionCookies;

#[get("/init")]
pub async fn get(
  session: SessionCookies,
  web::Query(Query { realm }): web::Query<Query>,
) -> Result<HttpResponse, Error> {
  if session.get_active_identity(realm).is_none() {
    return Ok(HttpResponse::Forbidden().finish());
  }

  Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct Query {
  realm: RealmId,
}
