use super::routes;
use async_trait::async_trait;
use hyper::{Body, Request, Response};
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
        let routes: Vec<Box<dyn Route + Send + Sync>> = vec![
            Box::<routes::About>::default(),
            Box::<routes::api::v1::newsletter::Post>::default(),
            Box::<routes::Socials>::default(),
            Box::<routes::NotFound>::default(),
            Box::<routes::Talks>::default(),
            Box::<routes::Sponsorships>::default(),
        ];

        Self::new(routes)
    }
}
