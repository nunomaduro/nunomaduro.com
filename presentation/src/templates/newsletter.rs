use askama::Template;
use domain::value_objects::Page;
use domain::value_objects::StaticPage;

#[derive(Template)]
#[template(path = "newsletter.html")]
pub struct NewsletterTemplate {
    page: StaticPage,
    url: String,
}

impl NewsletterTemplate {
    pub fn new(page: StaticPage, url: String) -> Self {
        Self { page, url }
    }
}
