use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use warp::Filter;

mod templates;
mod routes;

#[tokio::main]
async fn main() -> Result<()>{
    let pool = SqlitePool::connect("sqlite:dev.db").await?;
    let hbs= templates::init()?;

    let db_provider = warp::any().map(move || pool.clone());
    let root = warp::any().and(db_provider);

    let filters = routes::init(move || pool.clone(), move || hbs.clone());

    let server = warp::serve(filters).run(([0, 0, 0, 0], 8080));
    println!("Listening on 0.0.0.0:8080");
    server.await;
    Ok(())
}