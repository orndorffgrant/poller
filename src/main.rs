use clap::Clap;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::migrate;
use sqlx::prelude::*;
use sqlx::sqlite::SqlitePool;
use std::{env, time::Duration, path::Path};
use tide::{http::cookies::SameSite, log, sessions::SessionMiddleware};

mod routes;
mod session;
mod templates;


#[derive(Clone, Copy)]
pub struct Features {
    allow_participant_options: bool,
}

#[derive(Clone)]
pub struct State {
    db: SqlitePool,
    features: Features,
}

pub type Request = tide::Request<State>;

async fn assets_styles(_r: Request) -> tide::Result {
    let content = include_str!("../assets/styles.css");
    Ok(tide::Response::builder(200)
        .content_type(tide::http::mime::CSS)
        .body(content)
        .build())
}
async fn assets_charts(_r: Request) -> tide::Result {
    let content = include_str!("../assets/charts.css");
    Ok(tide::Response::builder(200)
        .content_type(tide::http::mime::CSS)
        .body(content)
        .build())
}
async fn assets_htmx(_r: Request) -> tide::Result {
    let content = include_str!("../assets/htmx.js");
    Ok(tide::Response::builder(200)
        .content_type(tide::http::mime::JAVASCRIPT)
        .body(content)
        .build())
}
async fn assets_alpine(_r: Request) -> tide::Result {
    let content = include_str!("../assets/alpine.js");
    Ok(tide::Response::builder(200)
        .content_type(tide::http::mime::JAVASCRIPT)
        .body(content)
        .build())
}

#[derive(FromRow)]
struct AdminUserQueryResult {
    _id: i64,
}

async fn init(db_path: String) -> tide::Result<SqlitePool> {
    // check if file exists yet and create it if it doesn't
    let db_file = Path::new(&db_path);
    if !db_file.exists() {
        std::fs::File::create(db_file)?;
    }

    // open the file as a sqlite db and migrate
    let db = SqlitePool::connect(&db_path).await?;
    migrate!().run(&db).await?;

    // create admin if doesn't exist
    let admin_opt = sqlx::query_as!(
        AdminUserQueryResult,
        r#"
        SELECT id AS _id FROM users WHERE role = "admin"
        "#
    )
    .fetch_optional(&db)
    .await?;

    match admin_opt {
        Some(_) => {
            log::info!("admin user exists")
        }
        None => {
            let rng = StdRng::from_entropy();
            let password: String = rng
                .sample_iter(&Alphanumeric)
                .take(14)
                .map(char::from)
                .collect();
            routes::users::create_user(&db, "admin", &password, "admin").await?;
            log::info!(
                "admin user created\nusername: admin\npassword: {}",
                password
            )
        }
    };

    Ok(db)
}

async fn run(db_path: String) -> tide::Result<SqlitePool> {
    let db = init(db_path).await?;

    let mut app = tide::with_state(State {
        db: db.clone(),
        features: Features {
            allow_participant_options: env::var("POLLER_ALLOW_PARTICIPANT_OPTIONS")
                == Ok("true".to_string()),
        },
    });

    let session_store = session::PollerSessionStore::from_client(db.clone());
    let cleanup_store = session_store.clone();
    async_std::task::spawn(async move {
        loop {
            async_std::task::sleep(Duration::from_secs(60 * 60)).await;
            if let Err(error) = cleanup_store.cleanup().await {
                log::error!("cleanup error: {}", error);
            }
        }
    });
    app.with(
        SessionMiddleware::new(session_store, env::var("POLLER_SECRET").unwrap().as_bytes())
            .with_cookie_name("poller.session.id")
            .with_same_site_policy(SameSite::Lax)
            .with_session_ttl(Some(Duration::from_secs(60 * 60))),
    );

    app.at("/assets/styles.css").get(assets_styles);
    app.at("/assets/charts.css").get(assets_charts);
    app.at("/assets/htmx.js").get(assets_htmx);
    app.at("/assets/alpine.js").get(assets_alpine);

    app.at("/").get(routes::home::root);

    let mut login = app.at("/login");
    login.get(routes::users::login_page);
    login.post(routes::users::login);
    app.at("/logout").get(routes::users::logout);
    app.at("/settings").get(routes::users::settings_page);
    app.at("/admin").get(routes::users::user_list_page);

    let mut users = app.at("/user");
    users.post(routes::users::new_user);
    users.at("/password").put(routes::users::change_my_password);

    let mut user = users.at("/:user_id");
    user.delete(routes::users::delete_user);
    user.at("/password")
        .put(routes::users::change_user_password);

    app.at("/polls").get(routes::polls::poll_list_page);
    let mut polls = app.at("/poll");
    polls.at("/new").get(routes::polls::new);

    let mut poll = polls.at("/:poll_id");
    poll.get(routes::polls::take_page);
    poll.put(routes::polls::edit_page_save);
    poll.delete(routes::polls::delete_poll);
    poll.at("/edit").get(routes::polls::edit_page);
    poll.at("/toggle-publish")
        .post(routes::polls::edit_page_toggle_publish);
    poll.at("/submission/single")
        .post(routes::polls::submit_single);
    poll.at("/submission/multi")
        .post(routes::polls::submit_multi);
    poll.at("/submission/score")
        .post(routes::polls::submit_score);
    poll.at("/results").get(routes::polls::results_page);

    let port = env::var("POLLER_PORT").unwrap_or("8000".to_string());
    app.listen(format!("0.0.0.0:{}", port)).await?;

    Ok(db)
}

#[derive(Clap)]
#[clap(version = "0.1.1")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Init(InitCommand),
    Run(RunCommand),
}

#[derive(Clap)]
struct InitCommand {
    db_path: String,
}

#[derive(Clap)]
struct RunCommand {
    db_path: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Info);
    let opts = Opts::parse();
    let db = match opts.subcmd {
        SubCommand::Init(i) => {
            init(i.db_path).await?
        }
        SubCommand::Run(r) => {
            run(r.db_path).await?
        }
    };
    db.close().await;
    Ok(())
}
