use std::sync::Arc;

use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;

pub struct WithTemplate<T: Serialize> {
    pub name: &'static str,
    pub value: T,
}

pub fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

pub fn init<'a>() -> Result<Arc<Handlebars<'a>>> {
    let mut hb = Handlebars::new();
    let home_template = include_str!("../templates/home.hbs");
    hb.register_template_string("home", home_template).unwrap();
    let poll_template = include_str!("../templates/poll.hbs");
    hb.register_template_string("poll", poll_template).unwrap();
    let poll_edit_template = include_str!("../templates/poll_edit.hbs");
    hb.register_template_string("poll_edit", poll_edit_template).unwrap();

    let hb = Arc::new(hb);
    // let hbs_provider = warp::any().map(move || hb.clone());
    let hbs_provider = move || hb.clone();

    Ok(hb)
}