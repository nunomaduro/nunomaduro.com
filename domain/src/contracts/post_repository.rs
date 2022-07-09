use crate::value_objects::PostPage;

pub trait PostRepository {
    fn all(&self) -> Vec<PostPage>;

    fn get(&self, id: &str) -> PostPage;
}
