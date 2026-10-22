use askama::Template;
use domain::value_objects::{Page, Post};

const LATEST_POSTS: usize = 3;

pub struct AboutPage {
    id: String,
    title: String,
    description: String,
}

impl AboutPage {
    pub fn new() -> Self {
        Self {
            id: "about".to_string(),
            title: "nunomaduro · open source, speaker, and content creator".to_string(),
            description: "open source developer, speaker, and content creator. the creator of pest, collision, termwind, and larastan."
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
#[template(path = "about.html")]
pub struct AboutTemplate {
    page: AboutPage,
    url: String,
    posts: Vec<Post>,
}

impl AboutTemplate {
    pub fn new(posts: Vec<Post>) -> Self {
        Self {
            page: AboutPage::new(),
            url: "/".to_string(),
            posts: posts.into_iter().take(LATEST_POSTS).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn post(slug: &str, title: &str, published_at: &str) -> Post {
        Post::new(
            slug.to_string(),
            title.to_string(),
            format!("the description of {title}"),
            String::new(),
            None,
            published_at.parse().unwrap(),
        )
    }

    #[test]
    fn renders_the_three_newest_posts_on_the_home_page() {
        let posts = vec![
            post("fourth", "Fourth Post Title", "April 10, 2026"),
            post("third", "Third Post Title", "March 3, 2026"),
            post("second", "Second Post Title", "February 2, 2026"),
            post("first", "First Post Title", "January 1, 2026"),
        ];

        let html = AboutTemplate::new(posts).render().unwrap();

        assert!(html.contains("Fourth Post Title"));
        assert!(html.contains("Third Post Title"));
        assert!(html.contains("Second Post Title"));
        assert!(html.contains("href=\"/posts/fourth\""));
        assert!(html.contains("April 10, 2026"));
        assert!(!html.contains("First Post Title"));
        assert!(!html.contains("href=\"/posts/first\""));
    }

    #[test]
    fn renders_the_home_page_when_no_post_exists() {
        let html = AboutTemplate::new(Vec::new()).render().unwrap();

        assert!(html.contains("no posts yet.. stay tuned."));
        assert!(!html.contains("href=\"/posts/"));
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = AboutTemplate::new(Vec::new()).render().unwrap();

        assert!(html.contains("nunomaduro<span class=\"text-amber-500\" aria-hidden=\"true\">.</span>"));
        assert!(html.contains("<h1 class=\"relative text-[40px] leading-none font-bold tracking-[-0.045em] sm:text-[44px]\">nunomaduro</h1>"));
        assert!(html.contains("&copy; nunomaduro"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }

    #[test]
    fn renders_each_social_account_with_its_number() {
        let html = AboutTemplate::new(Vec::new()).render().unwrap();

        assert_eq!(html.matches("data-count>").count(), 15);
        assert!(html.contains("data-stat=\"downloads\""));
        assert!(html.contains("data-stat=\"followers\""));
        assert!(html.contains("curl -fsSL nunomaduro.com/subscribe | sh"));
        assert!(html.contains("href=\"mailto:enunomaduro@gmail.com\""));
    }

    #[test]
    fn describes_the_home_page_as_the_profile_of_nunomaduro() {
        let html = AboutTemplate::new(Vec::new()).render().unwrap();
        let graph = crate::templates::structured_data(&html);
        let profile = crate::templates::node(&graph, "ProfilePage");
        let person = crate::templates::node(&graph, "Person");

        assert!(html.contains("<link rel=\"canonical\" href=\"https://nunomaduro.com/\">"));
        assert!(html.contains("<meta property=\"og:image\" content=\"https://nunomaduro.com/og/about.png\">"));
        assert_eq!(profile["url"], "https://nunomaduro.com/");
        assert_eq!(profile["mainEntity"]["@id"], "https://nunomaduro.com/#person");
        assert_eq!(person["givenName"], "Nuno");
        assert_eq!(person["familyName"], "Maduro");
        assert!(!html.contains("SearchAction"));
        assert!(person.get("worksFor").is_none());
        assert!(html.contains("open source&nbsp;· speaker&nbsp;· content creator</p>"));
        assert!(!html.to_lowercase().contains("staff"));
    }
}
