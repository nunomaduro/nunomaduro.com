use askama::Template;
use domain::value_objects::Page;
use domain::value_objects::StaticPage;

#[derive(Template)]
#[template(path = "support.html")]
pub struct SupportTemplate {
    page: StaticPage,
    url: String,
}

impl SupportTemplate {
    pub fn new(page: StaticPage, url: String) -> Self {
        Self { page, url }
    }
}
