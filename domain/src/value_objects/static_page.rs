use crate::value_objects::Page;

pub struct StaticPage {
    id: String,
    title: String,
    description: String,
    content: String,
}

impl StaticPage {
    pub fn new(id: String, title: String, description: String, content: String) -> Self {
        Self {
            id,
            title,
            description,
            content,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Page for StaticPage {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn description(&self) -> &str {
        &self.description
    }
}
