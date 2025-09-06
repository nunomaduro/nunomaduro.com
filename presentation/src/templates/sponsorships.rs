use askama::Template;

pub struct SponsorshipsPage {
    id: String,
    title: String,
    description: String,
}

impl SponsorshipsPage {
    pub fn new() -> Self {
        Self {
            id: "sponsorships".to_string(),
            title: "Nuno Maduro — Sponsorships".to_string(),
            description: "Supporting open source and content creation.".to_string(),
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
#[template(path = "support.html")]
pub struct SponsorshipsTemplate {
    page: SponsorshipsPage,
    url: String,
}

impl SponsorshipsTemplate {
    pub fn new() -> Self {
        Self {
            page: SponsorshipsPage::new(),
            url: "/sponsorships".to_string(),
        }
    }
}
