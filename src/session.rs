use async_session::Result;
use sqlx::{sqlite::SqlitePool, types::chrono::Utc};
use tide::{
    sessions::{Session, SessionStore},
    utils::async_trait,
};

#[derive(Clone, Debug)]
pub struct PollerSessionStore {
    db: SqlitePool,
}

impl PollerSessionStore {
    pub fn from_client(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn cleanup(&self) -> sqlx::Result<()> {
        let now = Utc::now().timestamp();
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE expires < ?1
            "#,
            now,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

struct SessionQueryResult {
    data: String,
}

#[async_trait]
impl SessionStore for PollerSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        let now = Utc::now().timestamp();

        let result: Option<SessionQueryResult> = sqlx::query_as!(
            SessionQueryResult,
            r#"
            SELECT data
            FROM sessions
            WHERE
                id = ?1
                AND (expires IS NULL OR expires > ?2)
            "#,
            id,
            now,
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(result
            .map(|session| serde_json::from_str(&session.data))
            .transpose()?)
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = session.id();
        let data = serde_json::to_string(&session)?;
        let expiry = session.expiry().map(|expiry| expiry.timestamp());

        sqlx::query!(
            r#"
            INSERT INTO sessions (
                id,
                data,
                expires
            )
            VALUES (?1, ?2, ?3)
            ON CONFLICT(id) DO
            UPDATE SET
              expires = excluded.expires,
              data = excluded.data
            "#,
            id,
            data,
            expiry,
        )
        .execute(&self.db)
        .await?;

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        let id = session.id();
        sqlx::query!(
            r#"
            DELETE FROM sessions WHERE id = ?
            "#,
            id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn clear_store(&self) -> Result {
        sqlx::query!(
            r#"
            DELETE FROM sessions
            "#,
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
}
