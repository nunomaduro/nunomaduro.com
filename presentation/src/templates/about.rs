use askama::Template;
use domain::value_objects::Page;
use domain::value_objects::StaticPage;

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    page: StaticPage,
}

impl AboutTemplate {
    pub fn new(page: StaticPage) -> Self {
        Self { page }
    }
}
