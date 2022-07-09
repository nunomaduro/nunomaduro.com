use crate::support::MarkdownFile;
use domain::contracts::PostRepository;
use domain::value_objects::PostPage;
use std::fs;

pub struct MarkdownPostRepository {
    path: String,
}

impl MarkdownPostRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn default() -> Self {
        Self::new("./content/posts".to_string())
    }
}

impl PostRepository for MarkdownPostRepository {
    fn all(&self) -> Vec<PostPage> {
        let files = fs::read_dir(self.path.to_owned()).unwrap();

        let mut vec = files
            .map(|file| file.unwrap().file_name())
            .map(|file_name| self.get(file_name.to_str().unwrap().replace(".md", "").as_str()))
            .collect::<Vec<PostPage>>();

        vec.sort_by(|a, b| b.id().cmp(&a.id()));

        vec
    }

    fn get(&self, id: &str) -> PostPage {
        let markdown = MarkdownFile::from_file(format!("{}/{}.md", self.path, id).as_str());

        PostPage::new(
            markdown.property("id").unwrap(),
            markdown.property("slug").unwrap(),
            markdown.property("type").unwrap(),
            markdown.property("title").unwrap(),
            markdown.property("description").unwrap(),
            markdown.property("published_at").unwrap(),
            markdown.property("duration").unwrap(),
            markdown.html(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let repository = MarkdownPostRepository::new("../content/posts".to_string());
        let pages = repository.all();

        assert!(pages.len() >= 1);
        assert_eq!(pages[0].title(), "How to install PHP 8.2 RC on Mac");
        assert_eq!(pages[0].r#type(), "Tutorial");
        assert_eq!(pages[0].published_at(), "July 20, 2022");
        assert_eq!(pages[0].duration(), "4 min read");
        assert!(pages[0].content().contains("<p>"));
    }

    #[test]
    fn test_get() {
        let repository = MarkdownPostRepository::new("../content/posts".to_string());
        let page = repository.get("1");

        assert_eq!(page.title(), "How to install PHP 8.2 RC on Mac");
        assert_eq!(page.r#type(), "Tutorial");
        assert_eq!(page.published_at(), "July 20, 2022");
        assert_eq!(page.duration(), "4 min read");
        assert!(page.content().contains("<p>"));
    }
}
