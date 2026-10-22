use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::PostRepository;
use crate::http::{Body, RequestBody};
use hyper::{Request, Response, StatusCode};
use infrastructure::repositories::MarkdownPostRepository;
use presentation::templates::{NotFoundTemplate, PostTemplate};

pub struct Post {
    slug: String,
}

impl Post {
    pub fn new(slug: String) -> Self {
        Self { slug }
    }
}

#[async_trait]
impl Route for Post {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        format!("/posts/{}", self.slug)
    }

    async fn handle(&self, _request: Request<RequestBody>) -> Response<Body> {
        let repository = MarkdownPostRepository::default();

        match repository.find(&self.slug) {
            Some(post) => {
                let template = PostTemplate::new(post);
                Response::new(template.render().unwrap().into())
            }
            None => {
                let mut response = Response::new(NotFoundTemplate::default().render().unwrap().into());
                *response.status_mut() = StatusCode::NOT_FOUND;
                response
            }
        }
    }
}
