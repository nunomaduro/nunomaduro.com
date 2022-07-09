use askama::Template;
use domain::value_objects::Page;
use domain::value_objects::StaticPage;

#[derive(Template)]
#[template(path = "talks.html")]
pub struct TalksTemplate {
    page: StaticPage,
}

impl TalksTemplate {
    pub fn new(page: StaticPage) -> Self {
        Self { page }
    }
}
