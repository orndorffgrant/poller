use async_sqlx_session::SqliteSessionStore;
use sqlx::sqlite::SqlitePool;
use std::{env, time::Duration};
use tide::{sessions::SessionMiddleware, Redirect};

mod templates;

mod routes;
mod utils;

#[derive(Clone)]
pub struct State {
    db: SqlitePool,
}

pub type Request = tide::Request<State>;

// async fn build_session_middleware(
//     db: SqlitePool,
// ) -> tide::Result<SessionMiddleware<SqliteSessionStore>> {
//     let session_store = SqliteSessionStore::from_client(db);
//     session_store.migrate().await?;
//     session_store.spawn_cleanup_task(Duration::from_secs(60 * 15));
//     let session_secret = env::var("TIDE_SECRET").unwrap();
//     Ok(SessionMiddleware::new(
//         session_store,
//         session_secret.as_bytes(),
//     ))
// }

async fn assets_styles(_r: Request) -> tide::Result {
    let content = include_str!("../assets/styles.css");
    Ok(
        tide::Response::builder(200)
        .content_type(tide::http::mime::CSS)
        .body(content)
        .build()
    )
}
async fn assets_htmx(_r: Request) -> tide::Result {
    let content = include_str!("../assets/htmx.js");
    Ok(
        tide::Response::builder(200)
        .content_type(tide::http::mime::JAVASCRIPT)
        .body(content)
        .build()
    )
}
async fn assets_alpine(_r: Request) -> tide::Result {
    let content = include_str!("../assets/alpine.js");
    Ok(
        tide::Response::builder(200)
        .content_type(tide::http::mime::JAVASCRIPT)
        .body(content)
        .build()
    )
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Info);
    let db = SqlitePool::connect("dev.db").await?;



    let mut app = tide::with_state(State { db: db.clone() });

    // app.with(build_session_middleware(db).await?);

    app.at("/assets/styles.css").get( assets_styles);
    app.at("/assets/htmx.js").get( assets_htmx);
    app.at("/assets/alpine.js").get( assets_alpine);

    app.at("/").get(Redirect::new("/home"));

    app.at("/home").get(routes::home::home);

    let mut polls = app.at("/poll");

    polls.at("/new").get(routes::polls::new);

    let mut poll = polls.at("/:poll_id");
    poll.get(routes::polls::take_page);
    poll.put(routes::polls::edit_page_save);
    poll.at("/edit").get(routes::polls::edit_page);
    poll.at("/toggle-publish").post(routes::polls::edit_page_toggle_publish);
    poll.at("/submission/single").post(routes::polls::submit_single);

    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
