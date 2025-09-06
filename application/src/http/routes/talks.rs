use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use hyper::{Body, Request, Response};
use presentation::templates::TalksTemplate;

pub struct Talks;

impl Default for Talks {
    fn default() -> Self {
        Self
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
        let template = TalksTemplate::new();

        Response::new(template.render().unwrap().into())
    }
}
