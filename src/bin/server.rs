use application::http::{Body, RequestBody, Router};
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("failed to bind {addr}: {e}");
            return;
        }
    };

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("failed to accept connection: {e}");
                continue;
            }
        };

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(e) = Builder::new(TokioExecutor::new())
                .serve_connection(io, service_fn(handle))
                .await
            {
                eprintln!("server error: {e}");
            }
        });
    }
}

async fn handle(request: Request<RequestBody>) -> Result<Response<Body>, Infallible> {
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
