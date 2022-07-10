use crate::http::Route;
use askama::Template;
use domain::contracts::StaticRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownStaticRepository;
use presentation::templates::TalksTemplate;

pub struct Talks {
    repository: Box<dyn StaticRepository>,
}

impl Talks {
    pub fn new(repository: Box<dyn StaticRepository>) -> Self {
        Self { repository }
    }

    pub fn default() -> Self {
        Self {
            repository: Box::new(MarkdownStaticRepository::default()),
        }
    }
}

impl Route for Talks {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/talks".to_string()
    }

    fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let talk = self.repository.get("talks");
        let template = TalksTemplate::new(talk, self.path());

        Response::new(template.render().unwrap().into())
    }
}
