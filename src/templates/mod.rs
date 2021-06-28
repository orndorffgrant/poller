pub mod polls;
pub mod users;

pub mod home {
    use askama::Template;
    #[derive(Template)]
    #[template(path = "home.html")]
    pub struct HomeTemplate {
        pub html_title: String,
    }
    #[derive(Template)]
    #[template(path = "404.html")]
    pub struct NotFoundTemplate {
        pub html_title: String,
    }
}
