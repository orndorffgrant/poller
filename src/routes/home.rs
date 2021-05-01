use std::sync::Arc;

use handlebars::Handlebars;
use serde_json::json;
use sqlx::SqlitePool;

use crate::templates::{WithTemplate, render};
pub async fn render_home(_pool: SqlitePool, hbs: Arc<Handlebars<'_>>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(render(WithTemplate {
        name: "home",
        value: json!({}),
    }, hbs))
}