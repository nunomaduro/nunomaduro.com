use askama::Template;
use domain::value_objects::PostPage;

#[derive(Template)]
#[template(path = "posts/show.html")]
pub struct ShowTemplate {
    page: PostPage,
    url: String,
}

impl ShowTemplate {
    pub fn new(page: PostPage, url: String) -> Self {
        Self { page, url }
    }
}
