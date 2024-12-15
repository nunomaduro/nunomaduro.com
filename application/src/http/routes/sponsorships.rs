use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::StaticRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownStaticRepository;
use presentation::templates::SponsorshipsTemplate;
use std::default::Default;

pub struct Sponsorships {
    repository: Box<dyn StaticRepository>,
}

unsafe impl Send for Sponsorships {}
unsafe impl Sync for Sponsorships {}

impl Sponsorships {
    pub fn new(repository: Box<dyn StaticRepository>) -> Self {
        Self { repository }
    }
}

impl Default for Sponsorships {
    fn default() -> Self {
        Self {
            repository: Box::<MarkdownStaticRepository>::default(),
        }
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
        let talk = self.repository.get("sponsorships");
        let template = SponsorshipsTemplate::new(talk, self.path());

        Response::new(template.render().unwrap().into())
    }
}
