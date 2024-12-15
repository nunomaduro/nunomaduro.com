use askama::Template;
use domain::value_objects::Page;
use domain::value_objects::StaticPage;

#[derive(Template)]
#[template(path = "support.html")]
pub struct SponsorshipsTemplate {
    page: StaticPage,
    url: String,
}

impl SponsorshipsTemplate {
    pub fn new(page: StaticPage, url: String) -> Self {
        Self { page, url }
    }
}
