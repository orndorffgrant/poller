use std::sync::Arc;

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::Filter;

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


#[tokio::main]
async fn main() {
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
    let handlebars = move |with_template| render(with_template, hb.clone());

    let view_poll = warp::path!("poll" / String)
        .map(|name| WithTemplate {
            name: "poll.hbs",
            value: json!({
                "poll_title" : name,
                "options": ["one", "two", "three"],
            }),
        })
        .map(handlebars.clone());
    let new_poll = warp::path!("poll" / "new")
        .map(|| WithTemplate {
            name: "poll_new.hbs",
            value: json!({
                "poll_id": "id",
                "poll_title": "temo",
                "options": ["one", "two", "three"],
                "require_name": true,
            }),
        })
        .map(handlebars.clone());

    let routes = new_poll.or(view_poll);

    let server = warp::serve(routes).run(([127, 0, 0, 1], 3030));
    println!("Listening on localhost:3030");
    server.await;
}