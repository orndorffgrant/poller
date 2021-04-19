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
    let template = include_str!("../templates/poll.html");

    let mut hb = Handlebars::new();
    // register the template
    hb.register_template_string("template.html", template)
        .unwrap();

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    let hello = warp::path!("poll" / String)
        .map(|name| WithTemplate {
            name: "template.html",
            value: json!({"user" : name}),
        })
        .map(handlebars);

    let server = warp::serve(hello).run(([127, 0, 0, 1], 3030));
    println!("Listening on localhost:3030");
    server.await;
}