pub mod polls;

pub mod home {
    use askama::Template;
    #[derive(Template)]
    #[template(path = "home.html")]
    pub struct HomeTemplate {}
    #[derive(Template)]
    #[template(path = "404.html")]
    pub struct NotFoundTemplate {}
}
