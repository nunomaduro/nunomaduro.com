use crate::http::Route;
use crate::http::{Body, RequestBody};
use askama::Template;
use async_trait::async_trait;
use hyper::{Request, Response};
use presentation::templates::PartnersTemplate;

pub struct Partners;

impl Default for Partners {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Route for Partners {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/partners".to_string()
    }

    async fn handle(&self, _request: Request<RequestBody>) -> Response<Body> {
        let template = PartnersTemplate::default();

        Response::new(template.render().unwrap().into())
    }
}
