use application::http::Router;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    // remove trailing slash from path
    let path = request.uri().path().trim_end_matches('/');
    let method = request.method().as_str();

    if std::env::var("APP_ENV").unwrap_or("production".to_string()) == "development".to_string() {
        if path.starts_with("/dist/") {
            let response = Response::new(Body::from(std::fs::read_to_string(
                format!("./public/{}", path)
            ).unwrap()));

            return Ok(response);
        }
    }

    let response = Router::default().route(method, path).handle(request);

    Ok(response)
}
