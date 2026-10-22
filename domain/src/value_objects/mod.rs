mod post;

pub use post::Post;

pub trait Page {
    fn id(&self) -> &str;

    fn title(&self) -> &str;

    fn description(&self) -> &str;
}
