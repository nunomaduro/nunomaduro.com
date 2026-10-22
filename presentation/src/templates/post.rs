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

#[cfg(test)]
mod tests {
    use super::*;

    fn post() -> Post {
        Post::new(
            "unique-slug".to_string(),
            "A Unique Post Title".to_string(),
            "A unique post description".to_string(),
            "<h2>a unique heading</h2>\n<p>a unique <code>paragraph</code></p>".to_string(),
            None,
            "March 3, 2026".to_string(),
        )
    }

    #[test]
    fn renders_the_html_of_the_post() {
        let html = PostTemplate::new(post()).render().unwrap();

        assert!(html.contains("<h2>a unique heading</h2>\n<p>a unique <code>paragraph</code></p>"));
        assert!(html.contains("A Unique Post Title</h1>"));
        assert!(html.contains("A unique post description"));
        assert!(html.contains("March 3, 2026"));
        assert!(html.contains("href=\"/posts\""));
        assert!(html.contains("thanks for reading."));
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = PostTemplate::new(post()).render().unwrap();

        assert!(html.contains("<title>nunomaduro · A Unique Post Title</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }
}
