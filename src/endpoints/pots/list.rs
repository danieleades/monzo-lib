use super::Pot;
use crate::{endpoints::handle_response, Result};
use serde::Deserialize;

/// An object representing a request to the Monzo API for a list of accounts
pub struct Request {
    request_builder: reqwest::RequestBuilder,
}

impl Request {
    pub(crate) fn new(http_client: &reqwest::Client, access_token: impl AsRef<str>) -> Self {
        let request_builder = http_client
            .get("https://api.monzo.com/accounts")
            .bearer_auth(access_token.as_ref());

        Self { request_builder }
    }

    /// Consume the request and return a response that will resolve to a list of
    /// pots
    pub async fn send(self) -> Result<Vec<Pot>> {
        /// A collection of Monzo pots
        #[derive(Deserialize)]
        struct Response {
            pots: Vec<Pot>,
        }

        let Response { pots } = handle_response(self.request_builder).await?;
        
        Ok(pots)
    }
}
