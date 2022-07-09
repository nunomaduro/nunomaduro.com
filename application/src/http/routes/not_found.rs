use crate::http::Route;
use hyper::{Body, Request, Response};

pub struct NotFound {
    // ..
}

impl NotFound {
    pub fn new() -> Self {
        Self {
            // ..
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}

impl Route for NotFound {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/404".to_string()
    }

    fn handle(&self, _request: Request<Body>) -> Response<Body> {
        Response::new("404 - not found.".into())
    }
}
