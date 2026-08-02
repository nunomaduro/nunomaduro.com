pub mod routes;

mod router;

pub use router::{Route, Router};

use bytes::Bytes;
use http_body_util::Full;

/// Body type used for every response we produce.
///
/// hyper 1.x no longer ships its own `Body`, so responses are built on
/// `http_body_util::Full` instead.
pub type Body = Full<Bytes>;

/// Body type hyper hands us for incoming requests.
pub type RequestBody = hyper::body::Incoming;
