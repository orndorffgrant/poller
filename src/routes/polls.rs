use std::sync::Arc;

use handlebars::Handlebars;
use rand::{Rng, distributions::Alphanumeric};
use serde_json::json;
use sqlx::SqlitePool;
use warp::{Filter, http::Uri};

use crate::templates::{WithTemplate, render};


pub async fn new_poll(pool: SqlitePool, _hbs: Arc<Handlebars<'_>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.acquire().await.map_err(|_err| warp::reject::reject())?;
    let new_id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    sqlx::query!(
        r#"
INSERT INTO polls (id, title)
VALUES (?1, ?2)
        "#,
        new_id,
        "New Poll"
    )
    .execute(&mut conn)
    .await.map_err(|_err| warp::reject())?;

    let uri: String = "/poll/".to_string() + &new_id + "/edit";
    Ok(warp::redirect::temporary(uri.parse::<Uri>().unwrap()))
}

pub async fn render_poll_edit(pool: SqlitePool, hbs: Arc<Handlebars<'_>>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.acquire().await.map_err(|_err| warp::reject())?;

    let p_opt = sqlx::query!(
        r#"
        SELECT id, title
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_optional(&mut conn)
    .await.map_err(|_err| warp::reject())?;

    if let Some(p) = p_opt {
        Ok(render(WithTemplate {
            name: "poll_edit",
            value: json!({
                "poll_title" : p.title,
                "options": ["one", "two", "three"],
            }),

        }, hbs))
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn render_poll(pool: SqlitePool, hbs: Arc<Handlebars<'_>>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.acquire().await.map_err(|_err| warp::reject())?;

    let p_opt = sqlx::query!(
        r#"
        SELECT id, title
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_optional(&mut conn)
    .await.map_err(|_err| warp::reject())?;

    if let Some(p) = p_opt {
        Ok(render(WithTemplate {
            name: "poll",
            value: json!({
                "poll_title" : p.title,
                "options": ["one", "two", "three"],
            }),

        }, hbs))
    } else {
        Err(warp::reject::not_found())
    }
}