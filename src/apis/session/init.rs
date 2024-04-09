use actix_web::*;
use fuzion_commons::db::PgClient;

use crate::dao::realm::RealmId;
use crate::domain::cookies::SessionCookies;
use crate::services::session::SessionService;

#[get("/init")]
pub async fn get(
  db_client: PgClient<'_>,
  session: SessionCookies,
  web::Query(Query { realm }): web::Query<Query>,
) -> Result<HttpResponse, Error> {
  let identity = match session.get_active_identity(realm) {
    Some(identity) => identity,
    _ => return Ok(HttpResponse::Forbidden().finish()),
  };

  let identity = SessionService::get_identity_dto(&db_client, &identity).await?;

  Ok(HttpResponse::Ok().json(identity))
}

#[derive(Deserialize)]
pub struct Query {
  realm: RealmId,
}
