use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use crate::http::{Body, RequestBody};
use hyper::{Request, Response};
use presentation::templates::MediaKitTemplate;

pub struct MediaKit;

impl Default for MediaKit {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Route for MediaKit {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/media-kit".to_string()
    }

    async fn handle(&self, _request: Request<RequestBody>) -> Response<Body> {
        let template = MediaKitTemplate::default();

        Response::new(template.render().unwrap().into())
    }
}
