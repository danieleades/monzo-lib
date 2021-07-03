use super::{Endpoint, Resolve};
use serde::Deserialize;

pub struct Request;

impl Endpoint for Request {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> &str {
        "https://api.monzo.com/ping/whoami"
    }
}

impl Resolve for Request {
    type Response = Response;

    fn resolve(&self, bytes: &[u8]) -> serde_json::Result<Self::Response> {
        serde_json::from_slice(bytes)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    authenticated: bool,
    client_id: String,
    user_id: String,
}
