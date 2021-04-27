use std::sync::Arc;

use anyhow::Result;
use handlebars::Handlebars;
use rand::{Rng, distributions::Alphanumeric};
use serde::Serialize;
use serde_json::json;
use sqlx::sqlite::SqlitePool;
use warp::{Filter, http::Uri};

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

async fn render_home(_pool: SqlitePool, hbs: Arc<Handlebars<'_>>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(render(WithTemplate {
        name: "home",
        value: json!({}),
    }, hbs))
}

async fn new_poll(pool: SqlitePool, _hbs: Arc<Handlebars<'_>>) -> Result<impl warp::Reply, warp::Rejection> {
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

async fn render_poll_edit(pool: SqlitePool, hbs: Arc<Handlebars<'_>>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
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

async fn render_poll(pool: SqlitePool, hbs: Arc<Handlebars<'_>>, id: String) -> Result<impl warp::Reply, warp::Rejection> {
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

#[tokio::main]
async fn main() -> Result<()>{
    let pool = SqlitePool::connect("sqlite:dev.db").await?;

    let mut hb = Handlebars::new();
    let home_template = include_str!("../templates/home.hbs");
    hb.register_template_string("home", home_template).unwrap();
    let poll_template = include_str!("../templates/poll.hbs");
    hb.register_template_string("poll", poll_template).unwrap();
    let poll_edit_template = include_str!("../templates/poll_edit.hbs");
    hb.register_template_string("poll_edit", poll_edit_template).unwrap();

    let hb = Arc::new(hb);
    let db_provider = warp::any().map(move || pool.clone());
    let hbs_provider = warp::any().map(move || hb.clone());
    let root = db_provider.and(hbs_provider);

    let home = root.clone().and(warp::path::end())
        .and_then(render_home);
    let view_poll = root.clone().and(warp::path!("poll" / String))
        .and_then(render_poll);
    let edit_poll = root.clone().and(warp::path!("poll" / String / "edit"))
        .and_then(render_poll_edit);
    let new_poll = root.clone().and(warp::path!("poll" / "new"))
        .and_then(new_poll);

    let routes = home
        .or(new_poll)
        .or(view_poll)
        .or(edit_poll);

    let server = warp::serve(routes).run(([127, 0, 0, 1], 3030));
    println!("Listening on localhost:3030");
    server.await;
    Ok(())
}