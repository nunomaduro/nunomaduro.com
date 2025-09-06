use crate::http::Route;
use async_trait::async_trait;
use hyper::{Body, Request, Response, StatusCode};
use hyper::header::{HeaderValue, LOCATION};

pub struct Socials;

impl Default for Socials {
    fn default() -> Self {
        Self
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
        // Redirect to the about page which now includes social links
        let mut response = Response::new(Body::empty());
        *response.status_mut() = StatusCode::MOVED_PERMANENTLY;
        response.headers_mut().insert(LOCATION, HeaderValue::from_static("/"));
        response
    }
}
