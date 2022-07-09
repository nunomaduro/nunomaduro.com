use domain::contracts::StaticRepository;
use domain::value_objects::StaticPage;
use pulldown_cmark::{html, Options, Parser};
use std::fs;

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
        let content = fs::read_to_string(format!("{}/{}.md", self.path, id)).unwrap();

        let options = Options::all();
        let parser = Parser::new_ext(content.as_str(), options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        StaticPage::new(id.to_string(), html_output.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let repository = MarkdownStaticRepository::new("../content".to_string());
        let page = repository.get("about");

        assert!(page.content().contains("Nuno Maduro is a speaker"));
    }
}
