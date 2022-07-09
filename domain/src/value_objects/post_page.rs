use crate::value_objects::Page;

pub struct PostPage {
    id: String,
    slug: String,
    r#type: String,
    title: String,
    description: String,
    published_at: String,
    duration: String,
    content: String,
}

impl PostPage {
    pub fn new(
        id: String,
        slug: String,
        r#type: String,
        title: String,
        description: String,
        published_at: String,
        duration: String,
        content: String,
    ) -> Self {
        Self {
            id,
            slug,
            r#type,
            title,
            description,
            published_at,
            duration,
            content,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn published_at(&self) -> &str {
        &self.published_at
    }

    pub fn duration(&self) -> &str {
        &self.duration
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Page for PostPage {
    fn id(&self) -> &str {
        &self.id
    }
}
