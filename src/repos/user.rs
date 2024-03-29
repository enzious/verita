use fuzion_commons::db::PgClient;

use crate::dao::realm::RealmId;
use crate::dao::user::{User, UserCredential, UserId};

use super::RepoError;

pub struct UserRepo;

impl UserRepo {
  pub async fn get_user(
    db_client: &PgClient<'_>,
    user_id: UserId,
  ) -> Result<Option<User>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_USER).await?;
    let rows = db_client.query(&stmt, &[&user_id]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn get_user_by_username(
    db_client: &PgClient<'_>,
    realm_id: RealmId,
    username: &str,
  ) -> Result<Option<User>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_USER_BY_USERNAME).await?;
    let rows = db_client.query(&stmt, &[&realm_id, &username]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn get_user_by_username_email(
    db_client: &PgClient<'_>,
    realm_id: RealmId,
    user: &str,
  ) -> Result<Option<User>, RepoError> {
    let stmt = db_client
      .prepare_cached(STMT_GET_USER_BY_USERNAME_OR_EMAIL)
      .await?;
    let rows = db_client.query(&stmt, &[&realm_id, &user]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn insert_user(db_client: &PgClient<'_>, user: &User) -> Result<User, RepoError> {
    let stmt = db_client.prepare_cached(STMT_INSERT_USER).await?;
    let rows = db_client
      .query(
        &stmt,
        &[
          &user.realm_id,
          &user.username,
          &user.email,
          &user.email_verified,
          &user.operator,
          &user.created,
          &user.updated,
        ],
      )
      .await?;
    rows
      .get(0)
      .map(|row| row.into())
      .ok_or(RepoError::InternalError)
  }

  pub async fn update_user(db_client: &PgClient<'_>, user: &User) -> Result<User, RepoError> {
    let stmt = db_client.prepare_cached(STMT_UPDATE_USER).await?;
    let rows = db_client
      .query(
        &stmt,
        &[
          &user.id,
          &user.realm_id,
          &user.username,
          &user.email,
          &user.email_verified,
          &user.operator,
          &user.created,
          &user.updated,
        ],
      )
      .await?;
    rows.get(0).map(|row| row.into()).ok_or(RepoError::NoRecord)
  }

  pub async fn get_operator_user(db_client: &PgClient<'_>) -> Result<Option<User>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_OPERATOR_USER).await?;
    let rows = db_client.query(&stmt, &[]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn get_user_credential_by_user_id(
    db_client: &PgClient<'_>,
    user_id: UserId,
  ) -> Result<Option<UserCredential>, RepoError> {
    let stmt = db_client
      .prepare_cached(STMT_GET_USER_CREDENTIAL_BY_USER_ID)
      .await?;
    let rows = db_client.query(&stmt, &[&user_id]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }

  pub async fn insert_user_credential(
    db_client: &PgClient<'_>,
    credential: &UserCredential,
  ) -> Result<UserCredential, RepoError> {
    let stmt = db_client
      .prepare_cached(STMT_INSERT_USER_CREDENTIAL)
      .await?;
    let rows = db_client
      .query(
        &stmt,
        &[
          &credential.user_id,
          &credential.credential_config_id,
          &credential.content,
          &credential.temporary,
          &credential.created,
          &credential.updated,
        ],
      )
      .await?;
    rows
      .get(0)
      .map(|row| row.into())
      .ok_or(RepoError::InternalError)
  }

  pub async fn delete_user_credential(
    db_client: &PgClient<'_>,
    user_id: UserId,
  ) -> Result<Option<UserCredential>, RepoError> {
    let stmt = db_client
      .prepare_cached(STMT_DELETE_USER_CREDENTIAL_BY_USER_ID)
      .await?;
    let rows = db_client.query(&stmt, &[&user_id]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }
}

static STMT_GET_USER: &'static str = r#"

SELECT *
FROM verita."user"
WHERE id = $1

"#;

static STMT_GET_USER_BY_USERNAME: &'static str = r#"

SELECT *
FROM verita."user"
WHERE realm_id = $1 AND username = $2

"#;

static STMT_GET_USER_BY_USERNAME_OR_EMAIL: &'static str = r#"

SELECT *
FROM verita."user"
WHERE realm_id = $1 AND (username = $2 OR email = $2)

"#;

static STMT_INSERT_USER: &'static str = r#"

INSERT INTO verita."user" (
  realm_id,
  username,
  email,
  email_verified,
  operator,
  created,
  updated
)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING *;

"#;

static STMT_UPDATE_USER: &'static str = r#"

UPDATE verita."user"
SET
  realm_id = $2,
  username = $3,
  email = $4,
  email_verified = $5,
  operator = $6,
  created = $7,
  updated = $8
WHERE id = $1
RETURNING *

"#;

static STMT_GET_OPERATOR_USER: &'static str = r#"

SELECT *
FROM verita."user"
WHERE operator = true

"#;

static STMT_GET_USER_CREDENTIAL_BY_USER_ID: &'static str = r#"

SELECT *
FROM verita.user_credential
WHERE user_id = $1

"#;

static STMT_INSERT_USER_CREDENTIAL: &'static str = r#"

INSERT INTO verita.user_credential (
  user_id,
  credential_config_id,
  content,
  temporary,
  created,
  updated
)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *;

"#;

static STMT_DELETE_USER_CREDENTIAL_BY_USER_ID: &'static str = r#"

DELETE FROM verita.user_credential
WHERE user_id = $1
RETURNING *

"#;
