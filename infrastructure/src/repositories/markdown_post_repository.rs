use crate::support::MarkdownFile;
use domain::contracts::PostRepository;
use domain::value_objects::{Post, PublicationDate};
use std::cmp::Reverse;
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

        posts.sort_by_key(|post| Reverse(post.published_at()));

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
        let published_at = publication_date(slug, &markdown.property("published_at")?);
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

fn publication_date(slug: &str, date: &str) -> PublicationDate {
    date.parse().unwrap_or_else(|_| {
        panic!("the post `{slug}` has the date `{date}`: write `published_at` in the form `January 8, 2026`")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_post(directory: &std::path::Path, slug: &str, published_at: &str) {
        let markdown = format!(
            "---\ntitle: {slug}\ndescription: {slug}\npublished_at: {published_at}\n---\nbody"
        );

        fs::write(directory.join(format!("{slug}.md")), markdown).unwrap();
    }

    #[test]
    fn sorts_the_posts_from_the_newest_to_the_oldest() {
        let directory = std::env::temp_dir().join(format!("posts-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        write_post(&directory, "winter-post", "December 31, 2025");
        write_post(&directory, "april-post", "April 10, 2026");
        write_post(&directory, "january-post", "January 8, 2026");
        write_post(&directory, "early-january-post", "January 4, 2026");

        let repository = MarkdownPostRepository::new(directory.to_str().unwrap().to_string());
        let slugs: Vec<String> = repository
            .all()
            .iter()
            .map(|post| post.slug().to_string())
            .collect();

        fs::remove_dir_all(&directory).unwrap();
        assert_eq!(
            slugs,
            ["april-post", "january-post", "early-january-post", "winter-post"]
        );
    }

    #[test]
    fn reads_the_date_of_each_post_of_the_site() {
        let repository = MarkdownPostRepository::new("../content/posts".to_string());

        repository.all();
    }

    #[test]
    #[should_panic(expected = "the post `wrong-date-post` has the date `2026-01-04`")]
    fn stops_on_a_post_with_a_date_in_a_different_form() {
        let directory = std::env::temp_dir().join(format!("posts-wrong-date-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        write_post(&directory, "wrong-date-post", "2026-01-04");

        MarkdownPostRepository::new(directory.to_str().unwrap().to_string()).find("wrong-date-post");
    }

    #[test]
    fn test_all_returns_empty_when_no_posts() {
        let repository = MarkdownPostRepository::new("./nonexistent".to_string());
        let posts = repository.all();
        assert!(posts.is_empty());
    }
}
