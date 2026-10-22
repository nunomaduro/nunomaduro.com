use askama::Template;

pub struct NotFoundPage {
    id: String,
    title: String,
    description: String,
}

impl NotFoundPage {
    pub fn new() -> Self {
        Self {
            id: "not-found".to_string(),
            title: "nunomaduro · page not found".to_string(),
            description: "this page does not exist".to_string(),
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
#[template(path = "not_found.html")]
pub struct NotFoundTemplate {
    page: NotFoundPage,
    url: String,
}

impl Default for NotFoundTemplate {
    fn default() -> Self {
        Self {
            page: NotFoundPage::new(),
            url: "/404".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_the_navigation_and_the_links_to_home_and_to_the_posts() {
        let html = NotFoundTemplate::default().render().unwrap();

        assert!(html.contains("<title>nunomaduro · page not found</title>"));
        assert!(html.contains("aria-label=\"main\""));
        assert!(html.contains("href=\"/posts\" class=\"inline-flex"));
        assert!(html.contains("href=\"/\" class=\"inline-flex"));
    }
}
