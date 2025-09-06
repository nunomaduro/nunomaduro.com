use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use hyper::{Body, Request, Response};
use presentation::templates::SponsorshipsTemplate;

pub struct Sponsorships;

impl Default for Sponsorships {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Route for Sponsorships {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/sponsorships".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let template = SponsorshipsTemplate::new();

        Response::new(template.render().unwrap().into())
    }
}
