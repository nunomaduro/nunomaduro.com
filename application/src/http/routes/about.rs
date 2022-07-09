use crate::http::Route;
use askama::Template;
use domain::contracts::StaticRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownStaticRepository;
use presentation::templates::AboutTemplate;

pub struct About {
    repository: Box<dyn StaticRepository>,
}

impl About {
    pub fn new(repository: Box<dyn StaticRepository>) -> Self {
        Self { repository }
    }
}

impl Default for About {
    fn default() -> Self {
        Self::new(Box::new(MarkdownStaticRepository::default()))
    }
}

impl Route for About {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "/about".to_string()
    }

    fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let article = self.repository.get("about");
        let template = AboutTemplate::new(article);

        Response::new(template.render().unwrap().into())
    }
}
