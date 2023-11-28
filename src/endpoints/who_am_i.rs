use serde::Deserialize;

use super::Endpoint;

pub struct Request;

impl Endpoint for Request {
    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn endpoint(&self) -> &str {
        "/ping/whoami"
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
