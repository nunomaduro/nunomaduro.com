use askama::Template;
use domain::value_objects::{Page, Post};

pub struct PostsPage {
    id: String,
    title: String,
    description: String,
}

impl PostsPage {
    pub fn new() -> Self {
        Self {
            id: "posts".to_string(),
            title: "nunomaduro · posts".to_string(),
            description:
                "articles about php, laravel, testing, open source, and software engineering"
                    .to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostsTemplate {
    page: PostsPage,
    url: String,
    posts: Vec<Post>,
}

impl PostsTemplate {
    pub fn new(posts: Vec<Post>) -> Self {
        Self {
            page: PostsPage::new(),
            url: "/posts".to_string(),
            posts,
        }
    }
}
