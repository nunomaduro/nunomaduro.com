use crate::value_objects::Post;

pub trait PostRepository {
    fn all(&self) -> Vec<Post>;
    fn find(&self, slug: &str) -> Option<Post>;
}
