pub mod polls;

pub mod home {
    use askama::Template;
    #[derive(Template)]
    #[template(path = "home.html")]
    pub struct HomeTemplate<'a> {
        name: &'a str,
    }

    impl<'a> HomeTemplate<'a> {
        pub fn new(name: &'a str) -> Self {
            Self { name }
        }
    }
}
