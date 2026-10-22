use crate::support::MarkdownFile;
use domain::contracts::PostRepository;
use domain::value_objects::Post;
use std::cmp::Reverse;
use std::fs;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

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

        posts.sort_by_cached_key(|post| Reverse(publication_date(post)));

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

fn publication_date(post: &Post) -> (u32, u32, u32) {
    parse_publication_date(post.published_at()).unwrap_or_else(|| {
        panic!(
            "the post `{}` has the date `{}`: write `published_at` in the form `January 8, 2026`",
            post.slug(),
            post.published_at()
        )
    })
}

fn parse_publication_date(date: &str) -> Option<(u32, u32, u32)> {
    let (month_and_day, year) = date.split_once(", ")?;
    let (month, day) = month_and_day.split_once(' ')?;
    let month = MONTHS.iter().position(|name| *name == month)? as u32 + 1;

    Some((year.parse().ok()?, month, day.parse().ok()?))
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
        let posts = repository.all();

        for post in posts {
            assert!(
                parse_publication_date(post.published_at()).is_some(),
                "{}",
                post.slug()
            );
        }
    }

    #[test]
    fn test_all_returns_empty_when_no_posts() {
        let repository = MarkdownPostRepository::new("./nonexistent".to_string());
        let posts = repository.all();
        assert!(posts.is_empty());
    }
}
