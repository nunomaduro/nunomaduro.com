use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::StaticRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownStaticRepository;
use presentation::templates::NewsletterTemplate;

pub struct Newsletter {
    repository: Box<dyn StaticRepository>,
}

unsafe impl Send for Newsletter {}
unsafe impl Sync for Newsletter {}

impl Newsletter {
    pub fn new(repository: Box<dyn StaticRepository>) -> Self {
        Self { repository }
    }
}

impl Default for Newsletter {
    fn default() -> Self {
        Self::new(Box::new(MarkdownStaticRepository::default()))
    }
}

#[async_trait]
impl Route for Newsletter {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/newsletter".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let article = self.repository.get("newsletter");
        let template = NewsletterTemplate::new(article, self.path());

        Response::new(template.render().unwrap().into())
    }
}
