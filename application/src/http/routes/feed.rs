use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::PostRepository;
use hyper::header::CONTENT_TYPE;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownPostRepository;
use presentation::templates::FeedTemplate;

pub struct Feed;

impl Default for Feed {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Route for Feed {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/feed.xml".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let repository = MarkdownPostRepository::default();
        let posts = repository.all();
        let template = FeedTemplate::new(posts);

        Response::builder()
            .header(CONTENT_TYPE, "application/rss+xml; charset=utf-8")
            .body(template.render().unwrap().into())
            .unwrap()
    }
}
