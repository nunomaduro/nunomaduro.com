use application::http::Router;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}

async fn handle(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    // remove trailing slash from path
    let path = request.uri().path().trim_end_matches('/');
    let method = request.method().as_str();

    if std::env::var("APP_ENV").unwrap_or_else(|_| "production".to_string()) == *"development"
        && (path.starts_with("/dist/") || path.starts_with("/profile."))
    {
        if let Ok(bytes) = std::fs::read(format!("./public/{path}")) {
            let content_type = match path.rsplit('.').next() {
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("webp") => "image/webp",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("svg") => "image/svg+xml",
                Some("woff2") => "font/woff2",
                _ => "application/octet-stream",
            };

            let response = Response::builder()
                .header("Content-Type", content_type)
                .body(Body::from(bytes))
                .unwrap();

            return Ok(response);
        }
    }

    let response = Router::default().route(method, path).handle(request).await;

    Ok(response)
}
