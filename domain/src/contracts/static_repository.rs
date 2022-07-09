use crate::value_objects::StaticPage;

pub trait StaticRepository {
    fn get(&self, id: &str) -> StaticPage;
}
