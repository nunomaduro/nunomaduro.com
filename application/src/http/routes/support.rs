use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::StaticRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownStaticRepository;
use presentation::templates::SupportTemplate;

pub struct Support {
    repository: Box<dyn StaticRepository>,
}

unsafe impl Send for Support {}
unsafe impl Sync for Support {}

impl Support {
    pub fn new(repository: Box<dyn StaticRepository>) -> Self {
        Self { repository }
    }

    pub fn default() -> Self {
        Self {
            repository: Box::new(MarkdownStaticRepository::default()),
        }
    }
}

#[async_trait]
impl Route for Support {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/support".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let talk = self.repository.get("support");
        let template = SupportTemplate::new(talk, self.path());

        Response::new(template.render().unwrap().into())
    }
}
