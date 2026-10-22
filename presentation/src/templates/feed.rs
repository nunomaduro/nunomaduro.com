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

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;

    #[test]
    fn writes_the_date_of_each_post_in_rfc_2822() {
        let post = Post::new(
            "unique-slug".to_string(),
            "A Unique Post Title".to_string(),
            "A unique post description".to_string(),
            String::new(),
            None,
            "January 4, 2026".parse().unwrap(),
        );
        let xml = FeedTemplate::new(vec![post]).render().unwrap();

        assert!(xml.contains("<pubDate>Sun, 04 Jan 2026 00:00:00 +0000</pubDate>"));
        assert!(xml.contains("<link>https://nunomaduro.com/posts/unique-slug</link>"));
    }
}
