use crate::value_objects::ListPage;
use crate::value_objects::PostPage;

pub trait PostRepository {
    fn all(&self) -> ListPage<PostPage>;

    fn get(&self, id: &str) -> PostPage;
}
