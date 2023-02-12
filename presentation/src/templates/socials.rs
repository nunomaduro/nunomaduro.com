use askama::Template;
use domain::value_objects::Page;
use domain::value_objects::StaticPage;

#[derive(Template)]
#[template(path = "socials.html")]
pub struct SocialsTemplate {
    page: StaticPage,
    url: String,
}

impl SocialsTemplate {
    pub fn new(page: StaticPage, url: String) -> Self {
        Self { page, url }
    }
}
