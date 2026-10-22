use askama::Template;

pub struct TalksPage {
    id: String,
    title: String,
    description: String,
}

impl TalksPage {
    pub fn new() -> Self {
        Self {
            id: "talks".to_string(),
            title: "nunomaduro · talks".to_string(),
            description: "conference talks about pest, laravel, and php tooling".to_string(),
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
#[template(path = "talks.html")]
pub struct TalksTemplate {
    page: TalksPage,
    url: String,
}

impl Default for TalksTemplate {
    fn default() -> Self {
        Self {
            page: TalksPage::new(),
            url: "/talks".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_each_featured_talk_as_a_lazy_video_with_its_name() {
        let html = TalksTemplate::default().render().unwrap();

        assert_eq!(html.matches("<iframe").count(), 8);
        assert_eq!(html.matches("loading=\"lazy\"").count(), 8);
        assert!(html.contains("title=\"strict ai engineering\" loading=\"lazy\""));
        assert!(!html.contains("title=\"YouTube video player\""));
    }

    #[test]
    fn renders_each_past_talk_as_a_link_to_its_video() {
        let html = TalksTemplate::default().render().unwrap();

        assert_eq!(html.matches("href=\"https://www.youtube.com/watch?v=").count(), 15);
        assert!(html.contains("href=\"https://www.youtube.com/watch?v=1c_p4JlMSgQ&amp;t=1800s\""));
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = TalksTemplate::default().render().unwrap();

        assert!(html.contains("<title>nunomaduro · talks</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }
}
