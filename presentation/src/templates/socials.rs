use askama::Template;

pub struct SocialsPage {
    id: String,
    title: String,
    description: String,
}

impl SocialsPage {
    pub fn new() -> Self {
        Self {
            id: "socials".to_string(),
            title: "Nuno Maduro — Socials".to_string(),
            description: "Connect with me across various platforms.".to_string(),
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
#[template(path = "socials.html")]
pub struct SocialsTemplate {
    page: SocialsPage,
    url: String,
}

impl SocialsTemplate {
    pub fn new() -> Self {
        Self {
            page: SocialsPage::new(),
            url: "/socials".to_string(),
        }
    }
}
