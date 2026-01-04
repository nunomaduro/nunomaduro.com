use askama::Template;
use domain::value_objects::{Page, Post};

#[derive(Template)]
#[template(path = "feed.xml", escape = "xml")]
pub struct FeedTemplate {
    posts: Vec<Post>,
}

impl FeedTemplate {
    pub fn new(posts: Vec<Post>) -> Self {
        Self { posts }
    }
}
