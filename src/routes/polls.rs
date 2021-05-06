use crate::templates::polls::*;
use crate::templates::home::NotFoundTemplate;
use rand::{Rng, distributions::Alphanumeric};
use tide::Redirect;
use serde::Deserialize;
use sqlx::prelude::*;

pub async fn new(request: crate::Request) -> tide::Result {
    let new_id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    sqlx::query!(
        r#"
        INSERT INTO polls (id, title, description)
        VALUES (?1, ?2, ?3)
        "#,
        new_id,
        "",
        ""
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

pub async fn edit_page(request: crate::Request) -> tide::Result {
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
}
pub async fn edit_page_save(mut request: crate::Request) -> tide::Result {
    let body: SavePollBody = request.body_json().await?;
    let id = request.param("poll_id")?;

    let require_name = body.require_name.unwrap_or(false);

    sqlx::query!(
        r#"
        UPDATE polls SET
        title = ?1,
        description = ?2,
        require_name = ?3
        WHERE id = ?4
        "#,
        body.title,
        body.description,
        require_name,
        id
    )
    .execute(&request.state().db)
    .await?;

    Ok(EditPageForm{
        id: id.to_string(),
        title: body.title,
        description: body.description,
        require_name: require_name
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