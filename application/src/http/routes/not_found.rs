use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use crate::http::{Body, RequestBody};
use hyper::{Request, Response};
use presentation::templates::NotFoundTemplate;

pub struct NotFound {
    // ..
}

unsafe impl Send for NotFound {}
unsafe impl Sync for NotFound {}

impl NotFound {
    pub fn new() -> Self {
        Self {
            // ..
        }
    }
}

impl Default for NotFound {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Route for NotFound {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/404".to_string()
    }

    async fn handle(&self, _request: Request<RequestBody>) -> Response<Body> {
        Response::builder()
            .status(404)
            .body(NotFoundTemplate::default().render().unwrap().into())
            .unwrap()
    }
}
