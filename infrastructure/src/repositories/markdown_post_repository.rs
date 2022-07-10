use crate::support::MarkdownFile;
use domain::contracts::PostRepository;
use domain::value_objects::ListPage;
use domain::value_objects::PostPage;
use std::fs;

pub struct MarkdownPostRepository {
    meta: String,
    path: String,
}

impl MarkdownPostRepository {
    pub fn new(meta: String, path: String) -> Self {
        Self { meta, path }
    }

    pub fn default() -> Self {
        Self::new("./content/posts".to_string(), "./content/posts".to_string())
    }
}

impl PostRepository for MarkdownPostRepository {
    fn all(&self) -> ListPage<PostPage> {
        let mut posts = fs::read_dir(&self.path)
            .unwrap()
            .map(|file| file.unwrap().file_name())
            .map(|file_name| self.get(file_name.to_str().unwrap().replace(".md", "").as_str()))
            .collect::<Vec<PostPage>>();

        posts.sort_by(|a, b| b.id().cmp(a.id()));

        let meta = MarkdownFile::from_file(format!("{}.md", self.meta).as_str());

        ListPage::new(
            "posts".to_string(),
            meta.property("title").unwrap(),
            meta.property("description").unwrap(),
            posts,
        )
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
        let repository = MarkdownPostRepository::new(
            "../content/posts".to_string(),
            "../content/posts".to_string(),
        );
        let mut pages = repository.all();

        assert!(pages.len() >= 1);
        let page = pages.next().unwrap();

        assert_eq!(page.title(), "Laravel Podcast Season 4 Episode 10");
        assert_eq!(page.r#type(), "Podcast");
        assert_eq!(page.published_at(), "June 24, 2022");
        assert_eq!(page.duration(), "25 min");
        assert!(page.content().contains("<p>"));
    }

    #[test]
    fn test_get() {
        let repository = MarkdownPostRepository::new(
            "../content/posts".to_string(),
            "../content/posts".to_string(),
        );
        let page = repository.get("1");

        assert_eq!(page.title(), "Laravel Podcast Season 4 Episode 10");
        assert_eq!(page.r#type(), "Podcast");
        assert_eq!(page.published_at(), "June 24, 2022");
        assert_eq!(page.duration(), "25 min");
        assert!(page.content().contains("<p>"));
    }
}
