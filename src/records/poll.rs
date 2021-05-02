use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteArguments;
type Query = sqlx::query::Query<'static, sqlx::Sqlite, SqliteArguments<'static>>;
type QueryAs<T> = sqlx::query::QueryAs<'static, sqlx::Sqlite, T, SqliteArguments<'static>>;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Poll {
    pub id: i64,
    pub text: String,
    pub title: String,
    created: i32,
    updated: i32,
}

impl crate::utils::AsRoute for Poll {
    fn as_route(&self) -> std::borrow::Cow<str> {
        format!("/poll/{}", self.id).into()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PartialPoll {
    pub text: Option<String>,
    pub title: Option<String>,
}

impl PartialPoll {
    pub fn update_by_id(&self, id: i64) -> Query {
        sqlx::query(
            "UPDATE polls (text, title, updated) VALUES (
            COALESCE($1, polls.text),
            COALESCE($2, polls.title),
            datetime('now')
          ) WHERE id = $3",
        )
        .bind(self.text.clone())
        .bind(self.title.clone())
        .bind(id)
    }

    pub fn create(&self) -> Query {
        sqlx::query(
            "INSERT INTO polls (text, title, created, updated) VALUES (
            $1, $2, DATETIME('now'), DATETIME('now')
          )",
        )
        .bind(self.text.clone())
        .bind(self.title.clone())
    }
}

impl Poll {
    pub fn all() -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM polls")
    }

    pub fn last_id() -> QueryAs<(i64,)> {
        sqlx::query_as("SELECT last_insert_rowid()")
    }

    pub fn find_by_id(id: i64) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM polls WHERE id = ?").bind(id)
    }

    pub fn delete_by_id(id: i64) -> Query {
        sqlx::query("DELETE FROM polls WHERE id = ?").bind(id)
    }

    // pub fn update(&self, partial: PartialPoll) -> Query {
    //     partial.update_by_id(self.id)
    // }
}
