use crate::http::Route;
use askama::Template;
use domain::contracts::PostRepository;
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
        let page = self.repository.all();

        let template = IndexTemplate::new(page, self.path());

        Response::new(template.render().unwrap().into())
    }
}
