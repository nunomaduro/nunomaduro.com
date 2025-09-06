use askama::Template;

pub struct AboutPage {
    id: String,
    title: String,
    description: String,
}

impl AboutPage {
    pub fn new() -> Self {
        Self {
            id: "about".to_string(),
            title: "Nuno Maduro — About".to_string(),
            description: "Developer, open-source enthusiast, and Laravel core team member.".to_string(),
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
}

impl AboutTemplate {
    pub fn new() -> Self {
        Self {
            page: AboutPage::new(),
            url: "".to_string(),
        }
    }
}
