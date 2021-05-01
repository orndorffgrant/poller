mod home;
mod polls;

pub fn init(root: impl Clone + warp::Filter) -> impl warp::Filter {

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