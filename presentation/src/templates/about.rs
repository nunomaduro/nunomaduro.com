use askama::Template;
use domain::value_objects::Page;
use domain::value_objects::StaticPage;

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    page: StaticPage,
    url: String,
}

impl AboutTemplate {
    pub fn new(page: StaticPage, url: String) -> Self {
        Self { page, url }
    }
}
