mod list_page;
mod post_page;
mod static_page;

pub use list_page::ListPage;
pub use post_page::PostPage;
pub use static_page::StaticPage;

pub trait Page {
    fn id(&self) -> &str;
}
