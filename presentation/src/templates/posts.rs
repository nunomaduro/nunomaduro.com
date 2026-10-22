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
                "articles by nunomaduro about php, laravel, pest, testing, open source, and software engineering"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_each_post_with_its_date_title_and_description() {
        let posts = vec![Post::new(
            "unique-slug".to_string(),
            "A Unique Post Title".to_string(),
            "A unique post description".to_string(),
            String::new(),
            None,
            "March 3, 2026".parse().unwrap(),
        )];

        let html = PostsTemplate::new(posts).render().unwrap();

        assert!(html.contains("href=\"/posts/unique-slug\""));
        assert!(html.contains("A Unique Post Title"));
        assert!(html.contains("A unique post description"));
        assert!(html.contains("March 3, 2026"));
    }

    #[test]
    fn renders_the_posts_page_when_no_post_exists() {
        let html = PostsTemplate::new(Vec::new()).render().unwrap();

        assert!(html.contains("no posts yet.. stay tuned."));
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = PostsTemplate::new(Vec::new()).render().unwrap();

        assert!(html.contains("<title>nunomaduro · posts</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }
}
