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

    pub fn route(&self, method: &str, path: &str) -> Box<dyn Route + Send + Sync> {
        // check for exact match first
        let route = self
            .routes
            .iter()
            .find(|route| route.method() == method && route.path() == path);

        if let Some(_) = route {
            return self
                .routes
                .iter()
                .find(|route| route.method() == method && route.path() == path)
                .map(|_| self.clone_route(method, path))
                .unwrap();
        }

        // check for dynamic post routes
        if method == "GET" && path.starts_with("/posts/") {
            let slug = path.trim_start_matches("/posts/");
            if !slug.is_empty() && !slug.contains('/') {
                return Box::new(routes::Post::new(slug.to_string()));
            }
        }

        // fallback to 404
        Box::<routes::NotFound>::default()
    }

    fn clone_route(&self, method: &str, path: &str) -> Box<dyn Route + Send + Sync> {
        // return a boxed route based on method and path
        match (method, path) {
            ("GET", "") => Box::<routes::About>::default(),
            ("GET", "/posts") => Box::<routes::Posts>::default(),
            ("GET", "/talks") => Box::<routes::Talks>::default(),
            ("GET", "/sponsorships") => Box::<routes::Sponsorships>::default(),
            ("GET", "/socials") => Box::<routes::Socials>::default(),
            ("GET", "/feed.xml") => Box::<routes::Feed>::default(),
            ("GET", "/sitemap.xml") => Box::<routes::Sitemap>::default(),
            ("POST", "/api/v1/newsletter") => Box::<routes::api::v1::newsletter::Post>::default(),
            _ => Box::<routes::NotFound>::default(),
        }
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
            Box::<routes::Posts>::default(),
            Box::<routes::Feed>::default(),
            Box::<routes::Sitemap>::default(),
        ];

        Self::new(routes)
    }
}
