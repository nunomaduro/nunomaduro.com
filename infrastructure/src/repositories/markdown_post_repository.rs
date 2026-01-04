use crate::support::MarkdownFile;
use domain::contracts::PostRepository;
use domain::value_objects::Post;
use std::fs;

pub struct MarkdownPostRepository {
    path: String,
}

impl MarkdownPostRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Default for MarkdownPostRepository {
    fn default() -> Self {
        Self::new("./content/posts".to_string())
    }
}

impl PostRepository for MarkdownPostRepository {
    fn all(&self) -> Vec<Post> {
        let path = std::path::Path::new(&self.path);

        if !path.exists() {
            return Vec::new();
        }

        let mut posts: Vec<Post> = fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                if path.extension()?.to_str()? != "md" {
                    return None;
                }

                let slug = path.file_stem()?.to_str()?.to_string();
                self.find(&slug)
            })
            .collect();

        // sort by published_at descending (newest first)
        posts.sort_by(|a, b| b.published_at().cmp(a.published_at()));

        posts
    }

    fn find(&self, slug: &str) -> Option<Post> {
        let file_path = format!("{}/{}.md", self.path, slug);

        if !std::path::Path::new(&file_path).exists() {
            return None;
        }

        let markdown = MarkdownFile::from_file(&file_path);

        let title = markdown.property("title")?;
        let description = markdown.property("description")?;
        let published_at = markdown.property("published_at")?;
        let image = markdown.property("image");

        Some(Post::new(
            slug.to_string(),
            title,
            description,
            markdown.html(),
            image,
            published_at,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_returns_empty_when_no_posts() {
        let repository = MarkdownPostRepository::new("./nonexistent".to_string());
        let posts = repository.all();
        assert!(posts.is_empty());
    }
}
