use std::sync::Arc;

use handlebars::Handlebars;
use sqlx::SqlitePool;
use warp::{Filter, Reply, Rejection};

mod home;
mod polls;

// pub fn init<F>(root: impl Filter<Extract = impl warp::Stream, Error = Rejection> + Clone) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
// pub fn init<F>(root: Tuple<Self = Self::Extract>::HList: Combine<Tuple<Self = F::Extract>::HList>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
// {

//     let home = root.clone().and(warp::path::end())
//         .and_then(home::render_home);

//     let new_poll = root.clone().and(warp::path!("poll" / "new"))
//         .and_then(polls::new_poll);
//     let edit_poll = root.clone().and(warp::path!("poll" / String / "edit"))
//         .and_then(polls::render_poll_edit);
//     let view_poll = root.clone().and(warp::path!("poll" / String))
//         .and_then(polls::render_poll);

//     home
//         .or(new_poll)
//         .or(view_poll)
//         .or(edit_poll)
// }
pub fn init<'a>(pool: impl Clone + Fn() -> SqlitePool, hbs: impl Clone + Fn() -> Arc<Handlebars<'a>>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    let db_provider = warp::any().map(pool);
    let hbs_provider = warp::any().map(hbs);
    let root = warp::any().and(db_provider).and(hbs_provider);

    let home = root.clone().and(warp::path::end())
        .and_then(home::render_home);

    let new_poll = root.clone().and(warp::path!("poll" / "new"))
        .and_then(polls::new_poll);
    let edit_poll = root.clone().and(warp::path!("poll" / String / "edit"))
        .and_then(polls::render_poll_edit);
    let view_poll = root.clone().and(warp::path!("poll" / String))
        .and_then(polls::render_poll);

    home
        .or(new_poll)
        .or(view_poll)
        .or(edit_poll)
}