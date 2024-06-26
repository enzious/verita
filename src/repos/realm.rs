use fuzion_commons::db::PgClient;

use crate::dao::realm::{Realm, RealmId};

use super::RepoError;

pub struct RealmRepo;

impl RealmRepo {
  pub async fn get_realm(
    db_client: &PgClient<'_>,
    realm_id: RealmId,
  ) -> Result<Option<Realm>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_REALM_BY_ID).await?;
    let rows = db_client.query(&stmt, &[&realm_id]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn get_realm_by_name(
    db_client: &PgClient<'_>,
    realm: &str,
  ) -> Result<Option<Realm>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_REALM_BY_NAME).await?;
    let rows = db_client.query(&stmt, &[&realm]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn get_realms(db_client: &PgClient<'_>) -> Result<Vec<Realm>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_REALMS).await?;
    let mut rows = db_client.query(&stmt, &[]).await?;
    Ok(rows.drain(..).map(|ref row| row.into()).collect::<Vec<_>>())
  }

  pub async fn insert_realm(db_client: &PgClient<'_>, realm: &Realm) -> Result<Realm, RepoError> {
    let stmt = db_client.prepare_cached(STMT_INSERT_REALM).await?;
    let rows = db_client
      .query(
        &stmt,
        &[&realm.name, &realm.operator, &realm.created, &realm.updated],
      )
      .await?;
    rows
      .get(0)
      .map(|row| row.into())
      .ok_or(RepoError::InternalError)
  }

  pub async fn get_operator_realm(db_client: &PgClient<'_>) -> Result<Option<Realm>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_OPERATOR_REALM).await?;
    let rows = db_client.query(&stmt, &[]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }
}

static STMT_GET_REALM_BY_ID: &'static str = r#"

SELECT *
FROM verita.realm
WHERE id = $1

"#;

static STMT_GET_REALM_BY_NAME: &'static str = r#"

SELECT *
FROM verita.realm
WHERE name = $1

"#;

static STMT_GET_REALMS: &'static str = r#"

SELECT *
FROM verita.realm

"#;

static STMT_GET_OPERATOR_REALM: &'static str = r#"

SELECT *
FROM verita.realm
WHERE operator = true

"#;

static STMT_INSERT_REALM: &'static str = r#"

INSERT INTO verita.realm (
  name,
  operator,
  created,
  updated
)
VALUES ($1, $2, $3, $4)
RETURNING *;

"#;
