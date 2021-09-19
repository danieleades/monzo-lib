use super::Endpoint;
use serde::Deserialize;

pub struct Request;

impl Endpoint for Request {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> &str {
        "https://api.monzo.com/ping/whoami"
    }
}

/// The response returned by the [`Client::who_am_i`](crate::Client::who_am_i)
/// method.
#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    /// Whether the current user is authenticated
    pub authenticated: bool,

    /// The client ID
    pub client_id: String,

    /// The unique identifier of the current user
    pub user_id: String,
}
