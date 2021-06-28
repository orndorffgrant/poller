use askama::Template;

#[derive(Template)]
#[template(path = "users/login_page.html")]
pub struct LoginPage {
    pub html_title: String,
}
