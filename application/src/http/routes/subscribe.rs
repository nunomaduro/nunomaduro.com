use crate::http::Route;
use async_trait::async_trait;
use hyper::header::CONTENT_TYPE;
use hyper::{Body, Request, Response};

pub struct Subscribe;

impl Default for Subscribe {
    fn default() -> Self {
        Self
    }
}

const SCRIPT: &str = include_str!("subscribe.sh");

#[async_trait]
impl Route for Subscribe {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/subscribe".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        Response::builder()
            .header(CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(Body::from(SCRIPT))
            .unwrap()
    }
}
