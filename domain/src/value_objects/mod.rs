mod post;
mod publication_date;

pub use post::Post;
pub use publication_date::PublicationDate;

pub trait Page {
    fn id(&self) -> &str;

    fn title(&self) -> &str;

    fn description(&self) -> &str;
}
