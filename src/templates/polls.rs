use askama::Template;

#[derive(Template)]
#[template(path = "polls/edit_page.html")]
pub struct EditPage {
    pub title: String,
}

#[derive(Template)]
#[template(path = "polls/take_page.html")]
pub struct TakePage {
    pub title: String,
}
