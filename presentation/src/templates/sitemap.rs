use askama::Template;
use domain::value_objects::Post;

#[derive(Template)]
#[template(path = "sitemap.xml", escape = "xml")]
pub struct SitemapTemplate {
    posts: Vec<Post>,
}

impl SitemapTemplate {
    pub fn new(posts: Vec<Post>) -> Self {
        Self { posts }
    }
}
