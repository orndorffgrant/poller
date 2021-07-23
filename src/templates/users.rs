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

#[derive(Template)]
#[template(path = "users/user_list_user_list.html")]
pub struct UserList {
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "users/settings_page.html")]
pub struct SettingsPage {
    pub html_title: String,
    pub name: String,
    pub changed_password: bool,
}

#[derive(Template)]
#[template(path = "users/settings_password_form.html")]
pub struct SettingsPasswordForm {
    pub changed_password: bool,
}