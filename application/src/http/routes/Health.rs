use crate::http::Route;
use async_trait::async_trait;
use hyper::{Body, Request, Response};

pub struct Health {
    //
}

unsafe impl Send for Health {}
unsafe impl Sync for Health {}

impl Health {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Route for Health {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/healthz-TGFyYXZlbCBDbG91ZCAK.php".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        // Just render 200 without template:
        Response::builder().status(200).body(Body::empty()).unwrap()
    }
}
