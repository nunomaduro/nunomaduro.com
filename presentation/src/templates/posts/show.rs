use askama::Template;
use domain::value_objects::PostPage;

#[derive(Template)]
#[template(path = "posts/show.html")]
pub struct ShowTemplate {
    page: PostPage,
}

impl ShowTemplate {
    pub fn new(page: PostPage) -> Self {
        Self { page }
    }
}
