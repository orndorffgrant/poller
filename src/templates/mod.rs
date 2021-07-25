pub mod polls;
pub mod users;

pub mod home {
    use askama::Template;
    #[derive(Template)]
    #[template(path = "404.html")]
    pub struct NotFoundTemplate {
        pub html_title: String,
    }
}
