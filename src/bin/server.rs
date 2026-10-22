use application::http::{Body, RequestBody, Router};
use hyper::service::service_fn;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use std::convert::Infallible;
use std::env::VarError;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::RangeInclusive;
use std::path::{Component, Path};
use tokio::net::{TcpListener, TcpStream};

const DEVELOPMENT_PORTS: RangeInclusive<u16> = 8000..=8010;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let listener = listen().await;

    let port = match listener.local_addr() {
        Ok(address) => address.port(),
        Err(error) => exit_with(&format!("cannot read the address of the listener: {error}")),
    };

    println!("listening on http://localhost:{port}");

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

async fn listen() -> TcpListener {
    match std::env::var("PORT") {
        Ok(port) => listen_on_port(&port).await,
        Err(VarError::NotPresent) => listen_on_free_development_port().await,
        Err(VarError::NotUnicode(_)) => exit_with("the variable [PORT] must be a number from [0] to [65535]"),
    }
}

async fn listen_on_port(port: &str) -> TcpListener {
    let Ok(number) = port.parse::<u16>() else {
        exit_with(&format!("the variable [PORT] must be a number from [0] to [65535], got [{port}]"));
    };

    if is_in_use(number).await {
        exit_with(&format!("cannot listen on port [{number}], because it is in use"));
    }

    match bind(number).await {
        Ok(listener) => listener,
        Err(error) => exit_with(&format!("cannot listen on port [{number}]: {error}")),
    }
}

async fn listen_on_free_development_port() -> TcpListener {
    for port in DEVELOPMENT_PORTS {
        if is_in_use(port).await {
            println!("port [{port}] is in use");
            continue;
        }

        if let Ok(listener) = bind(port).await {
            return listener;
        }

        println!("port [{port}] is in use");
    }

    exit_with(&format!(
        "cannot listen on ports [{}] to [{}], because each one is in use; set [PORT] to choose a different port",
        DEVELOPMENT_PORTS.start(),
        DEVELOPMENT_PORTS.end(),
    ))
}

async fn is_in_use(port: u16) -> bool {
    TcpStream::connect((Ipv4Addr::LOCALHOST, port)).await.is_ok()
        || TcpStream::connect((Ipv6Addr::LOCALHOST, port)).await.is_ok()
}

async fn bind(port: u16) -> io::Result<TcpListener> {
    match TcpListener::bind((Ipv6Addr::UNSPECIFIED, port)).await {
        Ok(listener) => Ok(listener),
        Err(_) => TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await,
    }
}

fn exit_with(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}

async fn handle(request: Request<RequestBody>) -> Result<Response<Body>, Infallible> {
    // remove trailing slash from path
    let path = request.uri().path().trim_end_matches('/').to_string();
    let method = request.method().to_string();

    let mut response = Router::default().route(&method, &path).handle(request).await;

    // routes without an explicit content type render html templates
    response
        .headers_mut()
        .entry(CONTENT_TYPE)
        .or_insert(HeaderValue::from_static("text/html; charset=utf-8"));

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
