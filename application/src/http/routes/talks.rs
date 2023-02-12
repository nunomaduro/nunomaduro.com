use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::StaticRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownStaticRepository;
use presentation::templates::TalksTemplate;
use std::default::Default;

pub struct Talks {
    repository: Box<dyn StaticRepository>,
}

unsafe impl Send for Talks {}
unsafe impl Sync for Talks {}

impl Talks {
    pub fn new(repository: Box<dyn StaticRepository>) -> Self {
        Self { repository }
    }
}

impl Default for Talks {
    fn default() -> Self {
        Self {
            repository: Box::<MarkdownStaticRepository>::default(),
        }
    }
}

#[async_trait]
impl Route for Talks {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/talks".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let talk = self.repository.get("talks");
        let template = TalksTemplate::new(talk, self.path());

        Response::new(template.render().unwrap().into())
    }
}
