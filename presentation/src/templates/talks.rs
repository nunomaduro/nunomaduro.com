use askama::Template;
use domain::value_objects::Page;
use domain::value_objects::StaticPage;

#[derive(Template)]
#[template(path = "talks.html")]
pub struct TalksTemplate {
    page: StaticPage,
    url: String,
}

impl TalksTemplate {
    pub fn new(page: StaticPage, url: String) -> Self {
        Self { page, url }
    }
}
