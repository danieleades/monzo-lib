use super::Transaction;
use crate::{
    client::{self, send_and_resolve_request},
    endpoints::Endpoint,
    Result,
};

/// A request to retrieve a list of transactions from the Monzo API
///
/// Use the builder-style methods to set optional fields on the request
#[derive(Debug)]
pub struct Request<'a> {
    client: &'a dyn client::Inner,
    endpoint: String,
    expand_merchant: bool,
}

impl<'a> Endpoint for Request<'a> {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    fn query(&self) -> Option<&dyn erased_serde::Serialize> {
        if self.expand_merchant {
            Some(&("expand[]", "merchant"))
        } else {
            None
        }
    }
}

impl<'a> Request<'a> {
    pub(crate) fn new(client: &'a dyn client::Inner, transaction_id: &str) -> Self {
        let endpoint = format!("https://api.monzo.com/transactions/{}", transaction_id);
        Self {
            client,
            endpoint,
            expand_merchant: false,
        }
    }

    /// Optionally expand the merchant field from an id string into a struct
    /// container merchant details
    pub fn expand_merchant(mut self) -> Self {
        self.expand_merchant = true;
        self
    }

    pub async fn send(self) -> Result<Transaction> {
        send_and_resolve_request(self.client, &self).await
    }
}
