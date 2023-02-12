use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::StaticRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownStaticRepository;
use presentation::templates::SocialsTemplate;

pub struct Socials {
    repository: Box<dyn StaticRepository>,
}

unsafe impl Send for Socials {}
unsafe impl Sync for Socials {}

impl Socials {
    pub fn new(repository: Box<dyn StaticRepository>) -> Self {
        Self { repository }
    }
}

impl Default for Socials {
    fn default() -> Self {
        Self::new(Box::<MarkdownStaticRepository>::default())
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
        let article = self.repository.get("socials");
        let template = SocialsTemplate::new(article, self.path());

        Response::new(template.render().unwrap().into())
    }
}
