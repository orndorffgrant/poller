use std::convert::TryInto;

use anyhow::Result;
use rand::{Rng, distributions::Alphanumeric};
use tide::Redirect;
use serde::Deserialize;
use sqlx::prelude::*;

use crate::templates::polls::*;
use crate::templates::home::NotFoundTemplate;

const POLL_TYPE_SINGLE: &str = "single";
const POLL_TYPE_MULTI: &str = "multi";
const POLL_TYPE_SCORE: &str = "score";

enum PollType {
    Single,
    Multi,
    Score,
}

impl PollType {
    fn from_string(str: &str) -> Result<PollType> {
        match str {
            POLL_TYPE_SINGLE => Ok(PollType::Single),
            POLL_TYPE_MULTI => Ok(PollType::Multi),
            POLL_TYPE_SCORE => Ok(PollType::Score),
            _ => Err(anyhow::Error::msg("invalid poll type")),
        }
    }
}

pub async fn new(request: crate::Request) -> tide::Result {
    let new_id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    sqlx::query!(
        r#"
        INSERT INTO polls (
            id,
            title,
            description,
            poll_type
        )
        VALUES (?1, ?2, ?3, ?4)
        "#,
        new_id,
        "",
        "",
        POLL_TYPE_SINGLE
    )
    .execute(&request.state().db)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO options (name, order_index, poll_id)
        VALUES
            (?1, ?2, ?7),
            (?3, ?4, ?7),
            (?5, ?6, ?7)
        "#,
        "Option 1",
        1024,
        "Option 2",
        2048,
        "Option 3",
        3072,
        new_id,
    )
    .execute(&request.state().db)
    .await?;

    let uri: String = "/poll/".to_string() + &new_id + "/edit";
    Ok(Redirect::temporary(uri).into())
}

#[derive(FromRow)]
struct Poll {
    id: String,
    title: String,
    description: String,
    require_name: bool,
    published: bool,
}

pub async fn take_page(request: crate::Request) -> tide::Result {
    let id = request.param("poll_id")?;
    let p_opt = sqlx::query_as!(
        Poll,
        r#"
        SELECT id, title, description, require_name, published
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_optional(&request.state().db)
    .await?;

    if let Some(p) = p_opt {
        Ok(TakePage{html_title: p.title.to_string(), title: p.title, require_name: p.require_name}.into())
    } else {
        Ok(tide::Response::builder(404)
            .body(NotFoundTemplate{html_title: "Not Found".to_string()}.to_string())
            .content_type(tide::http::mime::HTML)
            .build())
    }
}

#[derive(FromRow)]
struct EditPagePollQueryResult {
    id: String,
    title: String,
    description: String,
    require_name: bool,
    allow_participant_options: bool,
    poll_type: String,
    published: bool,
}
#[derive(FromRow)]
struct EditPageOptionQueryResult {
    id: i64,
    name: String,
}
pub async fn edit_page(request: crate::Request) -> tide::Result {
    let id = request.param("poll_id")?;
    let p_opt = sqlx::query_as!(
        EditPagePollQueryResult,
        r#"
        SELECT
            id,
            title,
            description,
            require_name,
            allow_participant_options,
            poll_type,
            published
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_optional(&request.state().db)
    .await?;

    if let Some(p) = p_opt {
        let options = sqlx::query_as!(
            EditPageOptionQueryResult,
            r#"
            SELECT
                id,
                name
            FROM options
            WHERE poll_id = ?1
            ORDER BY order_index
            "#,
            id
        ).fetch_all(&request.state().db)
        .await?;
        let html_title = if p.title.is_empty() {
            "New Poll"
        } else {
            &p.title
        };
        Ok(EditPage{
            html_title: html_title.to_string(),
            id: p.id,
            title: p.title,
            description: p.description,
            require_name: p.require_name,
            published: p.published,
            options: options.iter().map(|o| { EditPageOption{ id: o.id, name: o.name.to_owned() }}).collect(),
        }.into())
    } else {
        Ok(tide::Response::builder(404)
            .body(NotFoundTemplate{html_title: "Not Found".to_string()}.to_string())
            .content_type(tide::http::mime::HTML)
            .build())
    }
}

#[derive(Deserialize)]
struct SavePollBody {
    title: String,
    description: String,
    require_name: Option<bool>,
    allow_participant_options: Option<bool>,
    poll_type: String,
    options: Vec<String>,
}
pub async fn edit_page_save(mut request: crate::Request) -> tide::Result {
    let body: SavePollBody = request.body_json().await?;
    let id = request.param("poll_id")?;

    let require_name = body.require_name.unwrap_or(false);
    let allow_participant_options = body.allow_participant_options.unwrap_or(false);

    sqlx::query!(
        r#"
        UPDATE polls SET
        title = ?1,
        description = ?2,
        require_name = ?3,
        allow_participant_options = ?4,
        poll_type = ?5
        WHERE id = ?6
        "#,
        body.title,
        body.description,
        require_name,
        allow_participant_options,
        body.poll_type,
        id,
    )
    .execute(&request.state().db)
    .await?;

    sqlx::query!(
        r#"
        DELETE FROM options
        WHERE poll_id = ?1
        "#,
        id,
    )
    .execute(&request.state().db)
    .await?;

    for (index, name) in body.options.iter().enumerate() {
        let option_index: i64 = index.try_into()?;
        sqlx::query!(
            r#"
            INSERT INTO options (name, order_index, poll_id)
            VALUES (?1, ?2, ?3)
            "#,
            name,
            option_index,
            id,
        )
        .execute(&request.state().db)
        .await?;
    };


    let options = sqlx::query_as!(
        EditPageOptionQueryResult,
        r#"
        SELECT
            id,
            name
        FROM options
        WHERE poll_id = ?1
        ORDER BY order_index
        "#,
        id
    ).fetch_all(&request.state().db)
    .await?;

    Ok(EditPageForm{
        id: id.to_string(),
        title: body.title,
        description: body.description,
        require_name: require_name,
        options: options.iter().map(|o| { EditPageOption{ id: o.id, name: o.name.to_owned() }}).collect(),
    }.into())
}

struct Published {
    published: bool
}
pub async fn edit_page_toggle_publish(mut request: crate::Request) -> tide::Result {
    let id = request.param("poll_id")?;

    sqlx::query!(
        r#"
        UPDATE polls SET
        published = ((published | 1) - (published & 1))
        WHERE id = ?1
        "#,
        id
    )
    .execute(&request.state().db)
    .await?;

    let p = sqlx::query_as!(
        Published,
        r#"
        SELECT published
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_one(&request.state().db)
    .await?;

    Ok(EditPagePublish{
        id: id.to_string(),
        published: p.published,
    }.into())
}