use askama::Template;

#[derive(Template)]
#[template(path = "polls/edit_page.html")]
pub struct EditPage {
    pub html_title: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub require_name: bool,
}
#[derive(Template)]
#[template(path = "polls/edit_page_form.html")]
pub struct EditPageForm {
    pub id: String,
    pub title: String,
    pub description: String,
    pub require_name: bool,
}

#[derive(Template)]
#[template(path = "polls/take_page.html")]
pub struct TakePage {
    pub html_title: String,
    pub title: String,
    pub require_name: bool,
}
