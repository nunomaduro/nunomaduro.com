use askama::Template;
use domain::value_objects::{Page, Post};

pub struct PostPage {
    post: Post,
}

impl PostPage {
    pub fn new(post: Post) -> Self {
        Self { post }
    }

    pub fn id(&self) -> &str {
        "post"
    }

    pub fn title(&self) -> String {
        format!("nunomaduro · {}", self.post.title())
    }

    pub fn heading(&self) -> &str {
        self.post.title()
    }

    pub fn description(&self) -> &str {
        self.post.description()
    }

    pub fn slug(&self) -> &str {
        self.post.slug()
    }

    pub fn published_at(&self) -> &str {
        self.post.published_at()
    }
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    page: PostPage,
    url: String,
    content: String,
}

impl PostTemplate {
    pub fn new(post: Post) -> Self {
        let url = format!("/posts/{}", post.slug());
        let content = post.content().to_string();

        Self {
            page: PostPage::new(post),
            url,
            content,
        }
    }
}
