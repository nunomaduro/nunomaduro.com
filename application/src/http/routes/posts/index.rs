use crate::http::Route;
use askama::Template;
use domain::contracts::PostRepository;
use domain::value_objects::ListPage;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownPostRepository;
use presentation::templates::posts::IndexTemplate;

pub struct Index {
    repository: Box<dyn PostRepository>,
}

impl Index {
    pub fn new(repository: Box<dyn PostRepository>) -> Self {
        Self { repository }
    }

    pub fn default() -> Self {
        Self::new(Box::new(MarkdownPostRepository::default()))
    }
}

impl Route for Index {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "".to_string()
    }

    fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let posts = self.repository.all();

        let page = ListPage::new("posts".to_string(), posts);

        let template = IndexTemplate::new(page);

        Response::new(template.render().unwrap().into())
    }
}
