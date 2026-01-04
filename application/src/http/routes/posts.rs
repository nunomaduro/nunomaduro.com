use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::PostRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownPostRepository;
use presentation::templates::PostsTemplate;

pub struct Posts;

impl Default for Posts {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Route for Posts {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/posts".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let repository = MarkdownPostRepository::default();
        let posts = repository.all();
        let template = PostsTemplate::new(posts);

        Response::new(template.render().unwrap().into())
    }
}
