use askama::Template;
use domain::value_objects::ListPage;
use domain::value_objects::Page;
use domain::value_objects::PostPage;

#[derive(Template)]
#[template(path = "posts/index.html")]
pub struct IndexTemplate {
    page: ListPage<PostPage>,
}

impl IndexTemplate {
    pub fn new(page: ListPage<PostPage>) -> Self {
        Self { page }
    }
}
