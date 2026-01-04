use crate::value_objects::Page;

#[derive(Clone)]
pub struct Post {
    slug: String,
    title: String,
    description: String,
    content: String,
    image: Option<String>,
    published_at: String,
}

impl Post {
    pub fn new(
        slug: String,
        title: String,
        description: String,
        content: String,
        image: Option<String>,
        published_at: String,
    ) -> Self {
        Self {
            slug,
            title,
            description,
            content,
            image,
            published_at,
        }
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }

    pub fn published_at(&self) -> &str {
        &self.published_at
    }
}

impl Page for Post {
    fn id(&self) -> &str {
        &self.slug
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn description(&self) -> &str {
        &self.description
    }
}
