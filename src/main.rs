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

async fn new_poll(pool: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
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

    let uri: String = "/poll/".to_string() + &new_id;
    Ok(warp::redirect::temporary(uri.parse::<Uri>().unwrap()))
}

async fn render_poll(id: String, pool: SqlitePool, hbs: Arc<Handlebars<'_>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.acquire().await.map_err(|_err| warp::reject::reject())?;

    let p = sqlx::query!(
    // let p_opt = sqlx::query!(
        r#"
        SELECT id, title
        FROM polls
        WHERE id = ?1
        "#,
        id
    )
    .fetch_one(&mut conn)
    // .fetch_optional(&mut conn)
    .await.map_err(|_err| warp::reject())?;
    // probably failed above ^

    // if let Some(p) = p_opt {
        Ok(render(WithTemplate {
            name: "poll.hbs",
            value: json!({
                "poll_title" : p.title,
                "options": ["one", "two", "three"],
            }),

        }, hbs))
    // } else {
    //     Ok(warp::reply())
    // }
}

#[tokio::main]
async fn main() -> Result<()>{
    // create a poll
    let pool = SqlitePool::connect("sqlite:dev.db").await?;

    // real stuff
    let poll_template = include_str!("../templates/poll.hbs");
    let poll_new_template = include_str!("../templates/poll_new.hbs");

    let mut hb = Handlebars::new();
    // register the templates
    hb.register_template_string("poll.hbs", poll_template).unwrap();
    hb.register_template_string("poll_new.hbs", poll_new_template).unwrap();

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    // let handlebars = move |with_template| render(with_template, hb.clone());
    let provide_pool = move || pool.clone();

    let view_poll = warp::path!("poll" / String)
        .and(warp::any().map(provide_pool.clone()))
        .and(warp::any().map(move || hb.clone()))
        .and_then(render_poll);
    let new_poll = warp::path!("poll" / "new")
        .and(warp::any().map(provide_pool.clone()))
        .and_then(new_poll);

    let routes = new_poll.or(view_poll);

    let server = warp::serve(routes).run(([127, 0, 0, 1], 3030));
    println!("Listening on localhost:3030");
    server.await;
    Ok(())
}