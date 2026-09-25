use application::http::{Body, RequestBody, Router};
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::{Component, Path};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

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
    let path = request.uri().path().trim_end_matches('/').to_string();
    let method = request.method().to_string();

    let response = Router::default().route(&method, &path).handle(request).await;

    // fall back to files in ./public when no route matches
    if response.status() == 404 && method == "GET" {
        if let Some(file) = public_file(&path).await {
            return Ok(file);
        }
    }

    Ok(response)
}

async fn public_file(path: &str) -> Option<Response<Body>> {
    let relative = Path::new(path.trim_start_matches('/'));

    // only allow plain, non-hidden path segments to prevent directory traversal
    let safe = relative.components().all(|component| match component {
        Component::Normal(segment) => !segment.to_string_lossy().starts_with('.'),
        _ => false,
    });

    if !safe || relative.as_os_str().is_empty() {
        return None;
    }

    let bytes = tokio::fs::read(Path::new("./public").join(relative)).await.ok()?;

    let content_type = match path.rsplit('.').next() {
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("webp") => "image/webp",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("txt") => "text/plain",
        Some("xml") => "application/xml",
        Some("webmanifest") => "application/manifest+json",
        _ => "application/octet-stream",
    };

    Response::builder()
        .header("Content-Type", content_type)
        .body(Body::from(bytes))
        .ok()
}
