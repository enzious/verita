use fuzion_commons::db::PgClient;

use crate::dao::user::User;

use super::RepoError;

pub struct UserRepo;

impl UserRepo {
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
    rows.get(0).map(|row| row.into()).ok_or(RepoError::Internal)
  }

  pub async fn get_operator_user(db_client: &PgClient<'_>) -> Result<Option<User>, RepoError> {
    let stmt = db_client.prepare_cached(STMT_GET_OPERATOR_USER).await?;
    let rows = db_client.query(&stmt, &[]).await?;
    Ok(rows.get(0).map(|row| row.into()))
  }
}

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
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *;

"#;

static STMT_GET_OPERATOR_USER: &'static str = r#"

SELECT *
FROM verita."user"
WHERE operator = true

"#;
