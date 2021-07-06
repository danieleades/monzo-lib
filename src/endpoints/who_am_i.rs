use super::Endpoint;
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

#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    authenticated: bool,
    client_id: String,
    user_id: String,
}
