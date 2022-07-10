use crate::http::Route;
use askama::Template;
use domain::contracts::PostRepository;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use presentation::templates::posts::ShowTemplate;

pub struct Show {
    repository: Box<dyn PostRepository>,
    id: String,
    slug: String,
}

impl Show {
    pub fn new(repository: Box<dyn PostRepository>, id: String, slug: String) -> Self {
        Self {
            repository,
            id,
            slug,
        }
    }
}

impl Route for Show {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        format!("/{}", self.slug)
    }

    fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let page = self.repository.get(&self.id);

        let template = ShowTemplate::new(page, self.path());

        Response::new(template.render().unwrap().into())
    }
}
