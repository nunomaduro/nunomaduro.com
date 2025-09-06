mod static_page;

pub use static_page::StaticPage;

pub trait Page {
    fn id(&self) -> &str;

    fn title(&self) -> &str;

    fn description(&self) -> &str;
}
