use crate::support::MarkdownFile;
use domain::contracts::StaticRepository;
use domain::value_objects::StaticPage;

pub struct MarkdownStaticRepository {
    path: String,
}

impl MarkdownStaticRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn default() -> Self {
        Self::new("./content".to_string())
    }
}

impl StaticRepository for MarkdownStaticRepository {
    fn get(&self, id: &str) -> StaticPage {
        let markdown = MarkdownFile::from_file(format!("{}/{}.md", self.path, id).as_str());

        let title = markdown.property("title").unwrap();
        let description = markdown.property("description").unwrap();

        StaticPage::new(id.to_string(), title, description, markdown.html())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let repository = MarkdownStaticRepository::new("../content".to_string());
        let page = repository.get("about");

        assert!(page.content().contains("Nuno Maduro is a"));
    }
}
