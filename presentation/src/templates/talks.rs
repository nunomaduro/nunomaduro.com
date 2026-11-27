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
            title: "nunomaduro · Talks".to_string(),
            description: "Conference talks by nunomaduro about Pest, Laravel, and PHP tooling, from Laracon US, Laracon EU, Laracon India, Laracon AU, and more."
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
    fn renders_each_featured_talk_as_a_thumbnail_that_links_to_its_video() {
        let html = TalksTemplate::default().render().unwrap();
        let end = html.find("id=\"past-talks\"").unwrap();
        let cards: Vec<&str> = html[..end]
            .split("<a href=\"https://www.youtube.com/watch?v=")
            .skip(1)
            .collect();

        assert!(!html.contains("<iframe"));
        assert_eq!(cards.len(), 8);
        for card in cards {
            let video = &card[..card.find('"').unwrap()];
            let image = format!("src=\"/talk.{video}.jpg\"");

            assert!(card.contains(&image), "{video}");
            assert!(
                std::path::Path::new("../public")
                    .join(format!("talk.{video}.jpg"))
                    .exists(),
                "{video}"
            );
            assert!(card.contains("loading=\"lazy\""), "{video}");
        }
        assert!(html.contains("alt=\"Thumbnail of Strict AI engineering\""));
    }

    #[test]
    fn renders_each_past_talk_as_a_link_to_its_video() {
        let html = TalksTemplate::default().render().unwrap();
        let start = html.find("id=\"past-talks\"").unwrap();

        assert_eq!(
            html[start..]
                .matches("href=\"https://www.youtube.com/watch?v=")
                .count(),
            15
        );
        assert!(html.contains("href=\"https://www.youtube.com/watch?v=1c_p4JlMSgQ&amp;t=1800s\""));
    }

    #[test]
    fn writes_nunomaduro_as_one_word() {
        let html = TalksTemplate::default().render().unwrap();

        assert!(html.contains("<title>nunomaduro · Talks</title>"));
        assert!(!html.to_lowercase().contains("nuno maduro"));
    }

    #[test]
    fn describes_the_talks_page_as_a_collection_with_a_breadcrumb() {
        let html = TalksTemplate::default().render().unwrap();
        let graph = crate::templates::structured_data(&html);
        let page = crate::templates::node(&graph, "CollectionPage");

        assert!(html.contains("<link rel=\"canonical\" href=\"https://nunomaduro.com/talks\">"));
        assert_eq!(page["breadcrumb"]["itemListElement"][1]["name"], "Talks");
        assert_eq!(
            page["breadcrumb"]["itemListElement"][1]["item"],
            "https://nunomaduro.com/talks"
        );
    }
}
