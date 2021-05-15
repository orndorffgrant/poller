use askama::Template;

pub struct EditPageOption {
    pub id: i64,
    pub name: String,
}
#[derive(Template)]
#[template(path = "polls/edit_page.html")]
pub struct EditPage {
    pub html_title: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub require_name: bool,
    pub published: bool,
    pub options: Vec<EditPageOption>,
}
#[derive(Template)]
#[template(path = "polls/edit_page_form.html")]
pub struct EditPageForm {
    pub id: String,
    pub title: String,
    pub description: String,
    pub require_name: bool,
    pub options: Vec<EditPageOption>,
}
#[derive(Template)]
#[template(path = "polls/edit_page_form_options.html")]
pub struct EditPageFormOptions {
    pub id: String,
    pub options: Vec<EditPageOption>,
}
#[derive(Template)]
#[template(path = "polls/edit_page_publish.html")]
pub struct EditPagePublish {
    pub id: String,
    pub published: bool,
}

#[derive(Template)]
#[template(path = "polls/take_page.html")]
pub struct TakePage {
    pub html_title: String,
    pub title: String,
    pub require_name: bool,
}
