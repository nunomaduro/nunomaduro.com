use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use hyper::{Body, Request, Response};
use presentation::templates::SocialsTemplate;

pub struct Socials;

impl Default for Socials {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Route for Socials {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/socials".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let template = SocialsTemplate::new();

        Response::new(template.render().unwrap().into())
    }
}
