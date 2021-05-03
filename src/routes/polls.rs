use crate::templates::polls::*;
use crate::templates::home::NotFoundTemplate;
use rand::{Rng, distributions::Alphanumeric};
use tide::Redirect;
use sqlx::prelude::*;

pub async fn new(request: crate::Request) -> tide::Result {
    let new_id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    sqlx::query!(
        r#"
        INSERT INTO polls (id, title)
        VALUES (?1, ?2)
        "#,
        new_id,
        "New Poll"
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
    require_name: bool,
}

pub async fn take_page(request: crate::Request) -> tide::Result {
    let id = request.param("poll_id")?;
    let p_opt = sqlx::query_as!(
        Poll,
        r#"
        SELECT id, title, require_name
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
        SELECT id, title, require_name
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_optional(&request.state().db)
    .await?;

    if let Some(p) = p_opt {
        Ok(EditPage{
            html_title: p.title.to_string(),
            id: p.id,
            title: p.title,
            require_name: p.require_name
        }.into())
    } else {
        Ok(tide::Response::builder(404)
            .body(NotFoundTemplate{html_title: "Not Found".to_string()}.to_string())
            .content_type(tide::http::mime::HTML)
            .build())
    }
}