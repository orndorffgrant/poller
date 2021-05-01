use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use warp::Filter;

mod templates;
mod routes;

#[tokio::main]
async fn main() -> Result<()>{
    let pool = SqlitePool::connect("sqlite:dev.db").await?;

    let hb_provider = templates::init()?;
    let db_provider = warp::any().map(move || pool.clone());
    let root = db_provider.and(warp::any().map(hb_provider));

    let filters = routes::init(root);

    let server = warp::serve(filters).run(([0, 0, 0, 0], 8080));
    println!("Listening on 0.0.0.0:8080");
    server.await;
    Ok(())
}