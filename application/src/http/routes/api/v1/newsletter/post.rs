use crate::http::Route;
use async_trait::async_trait;
use domain::contracts::NewsletterRepository;
use hyper::body::to_bytes;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MailcoachNewsletterRepository;

pub struct Post {
    repository: Box<dyn NewsletterRepository + Send + Sync>,
}

unsafe impl Send for Post {}
unsafe impl Sync for Post {}

impl Post {
    pub fn new(repository: Box<dyn NewsletterRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn default() -> Self {
        Self::new(Box::new(MailcoachNewsletterRepository::default()))
    }
}

#[async_trait]
impl Route for Post {
    fn method(&self) -> String {
        "POST".to_string()
    }

    fn path(&self) -> String {
        "/api/v1/newsletter".to_string()
    }

    async fn handle(&self, request: Request<Body>) -> Response<Body> {
        let payload =
            String::from_utf8(to_bytes(request.into_body()).await.unwrap().to_vec()).unwrap();

        let subscriber: serde_json::Value = serde_json::from_str(payload.as_ref()).unwrap();

        let email = subscriber["email"].as_str().unwrap().to_string();

        let _ = self.repository.add(&email).await;

        return Response::builder()
            .status(201)
            .body(Body::from(""))
            .unwrap();
    }
}
