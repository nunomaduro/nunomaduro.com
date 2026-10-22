use askama::Template;
use domain::value_objects::{Page, Post, PublicationDate};

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
        format!("{} · nunomaduro", self.post.title())
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

    pub fn published_at(&self) -> PublicationDate {
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
            "March 3, 2026".parse().unwrap(),
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
    fn shares_the_post_as_an_article_with_its_own_social_card() {
        let html = PostTemplate::new(post()).render().unwrap();

        assert!(html.contains("<meta property=\"og:type\" content=\"article\">"));
        assert!(html.contains("<meta property=\"og:title\" content=\"A Unique Post Title\">"));
        assert!(html.contains("<meta property=\"og:image\" content=\"https://nunomaduro.com/og/posts/unique-slug.png\">"));
        assert!(html.contains("<meta property=\"article:published_time\" content=\"2026-03-03\">"));
        assert!(html.contains("<link rel=\"canonical\" href=\"https://nunomaduro.com/posts/unique-slug\">"));
        assert!(html.contains("<time datetime=\"2026-03-03\">March 3, 2026</time>"));
    }

    #[test]
    fn describes_the_post_as_a_blog_posting_in_valid_structured_data() {
        let post = Post::new(
            "quoted-slug".to_string(),
            "Pest's \"withHost\" </script> & more".to_string(),
            "A description with a \"quote\"".to_string(),
            String::new(),
            None,
            "January 4, 2026".parse().unwrap(),
        );
        let html = PostTemplate::new(post).render().unwrap();
        let graph = crate::templates::structured_data(&html);
        let article = crate::templates::node(&graph, "BlogPosting");
        let breadcrumb = crate::templates::node(&graph, "BreadcrumbList");

        assert_eq!(article["headline"], "Pest's \"withHost\" </script> & more");
        assert_eq!(article["description"], "A description with a \"quote\"");
        assert_eq!(article["datePublished"], "2026-01-04");
        assert_eq!(article["image"]["url"], "https://nunomaduro.com/og/posts/quoted-slug.png");
        assert_eq!(article["author"]["@id"], "https://nunomaduro.com/#person");
        assert_eq!(breadcrumb["itemListElement"][2]["item"], "https://nunomaduro.com/posts/quoted-slug");
    }

    #[test]
    fn gives_each_post_of_the_site_its_social_card() {
        for entry in std::fs::read_dir("../content/posts").into_iter().flatten() {
            let path = entry.unwrap().path();

            if path.extension().is_some_and(|extension| extension == "md") {
                let slug = path.file_stem().unwrap().to_str().unwrap();
                let card = format!("../public/og/posts/{slug}.png");

                assert!(std::path::Path::new(&card).exists(), "run `npm run og` to write {card}");
            }
        }
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = PostTemplate::new(post()).render().unwrap();

        assert!(html.contains("<title>A Unique Post Title · nunomaduro</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }
}
