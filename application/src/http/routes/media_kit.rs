use crate::http::Route;
use crate::http::{Body, RequestBody};
use async_trait::async_trait;
use hyper::header::{HeaderValue, LOCATION};
use hyper::{Request, Response, StatusCode};

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
        let mut response = Response::new(Body::default());
        *response.status_mut() = StatusCode::MOVED_PERMANENTLY;
        response
            .headers_mut()
            .insert(LOCATION, HeaderValue::from_static("/sponsorships"));
        response
    }
}
