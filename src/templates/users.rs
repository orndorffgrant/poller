use askama::Template;
use serde::Serialize;

#[derive(Template)]
#[template(path = "users/login_page.html")]
pub struct LoginPage {
    pub html_title: String,
    pub wrong: bool,
}
#[derive(Template)]
#[template(path = "users/login_page_form.html")]
pub struct LoginPageForm {
    pub wrong: bool,
}

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}
#[derive(Template)]
#[template(path = "users/user_list_page.html")]
pub struct UserListPage {
    pub html_title: String,
    pub users: Vec<User>,
}