use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use hyper::{Body, Request, Response};
use presentation::templates::AboutTemplate;

pub struct About;

impl Default for About {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Route for About {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let template = AboutTemplate::new();

        Response::new(template.render().unwrap().into())
    }
}
