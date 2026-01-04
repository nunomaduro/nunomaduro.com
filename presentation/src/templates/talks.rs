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
            description: "staff software engineer at laravel · speaker · content creator"
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

impl TalksTemplate {
    pub fn new() -> Self {
        Self {
            page: TalksPage::new(),
            url: "/talks".to_string(),
        }
    }
}
