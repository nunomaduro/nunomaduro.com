use super::routes;
use async_trait::async_trait;
use domain::contracts::PostRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownPostRepository;
use std::default::Default;

pub struct Router {
    routes: Vec<Box<dyn Route + Send + Sync>>,
}

#[async_trait]
pub trait Route {
    fn method(&self) -> String;
    fn path(&self) -> String;
    async fn handle(&self, request: Request<Body>) -> Response<Body>;
}

impl Router {
    pub fn new(routes: Vec<Box<dyn Route + Send + Sync>>) -> Self {
        Self { routes }
    }

    pub fn route(&self, method: &str, path: &str) -> &(dyn Route + Send + Sync) {
        let route = self
            .routes
            .iter()
            .find(|route| route.method() == method && route.path() == path);

        match route {
            Some(route) => route,
            None => self
                .routes
                .iter()
                .find(|route| route.method() == "GET" && route.path() == "/404")
                .unwrap(),
        }
        .as_ref()
    }
}

impl Default for Router {
    fn default() -> Self {
        let mut routes: Vec<Box<dyn Route + Send + Sync>> = vec![
            Box::<routes::About>::default(),
            Box::<routes::Health>::default(),
            Box::<routes::api::v1::newsletter::Post>::default(),
            Box::<routes::Socials>::default(),
            Box::<routes::NotFound>::default(),
            Box::<routes::Talks>::default(),
            Box::<routes::Sponsorships>::default(),
        ];

        let posts = MarkdownPostRepository::default().all();

        for post in posts {
            routes.push(Box::new(routes::posts::Show::new(
                Box::<MarkdownPostRepository>::default(),
                post.id().to_string(),
                post.slug().to_string(),
            )));
        }

        Self::new(routes)
    }
}
