use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::PostRepository;
use crate::http::{Body, RequestBody};
use hyper::{Request, Response};
use infrastructure::repositories::MarkdownPostRepository;
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

    async fn handle(&self, _request: Request<RequestBody>) -> Response<Body> {
        let repository = MarkdownPostRepository::default();
        let template = AboutTemplate::new(repository.all());

        Response::new(template.render().unwrap().into())
    }
}
