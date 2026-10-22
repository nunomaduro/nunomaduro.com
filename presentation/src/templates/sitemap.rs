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

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;

    #[test]
    fn writes_the_date_of_each_post_as_its_last_modification() {
        let post = Post::new(
            "unique-slug".to_string(),
            "A Unique Post Title".to_string(),
            "A unique post description".to_string(),
            String::new(),
            None,
            "January 4, 2026".parse().unwrap(),
        );
        let xml = SitemapTemplate::new(vec![post]).render().unwrap();

        assert!(xml.contains("<loc>https://nunomaduro.com/posts/unique-slug</loc>\n    <lastmod>2026-01-04</lastmod>"));
    }
}
