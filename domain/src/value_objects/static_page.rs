use crate::value_objects::Page;

pub struct StaticPage {
    id: String,
    content: String,
}

impl StaticPage {
    pub fn new(id: String, content: String) -> Self {
        Self { id, content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Page for StaticPage {
    fn id(&self) -> &str {
        &self.id
    }
}
