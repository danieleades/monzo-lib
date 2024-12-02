use super::Transaction;
use crate::{client, endpoints::Endpoint, Result};

/// A request to retrieve a list of transactions from the Monzo API
///
/// Use the builder-style methods to set optional fields on the request
#[derive(Debug)]
#[must_use]
pub struct Request<'a, C>
where
    C: client::Inner,
{
    client: &'a C,
    endpoint: String,
    expand_merchant: bool,
}

impl<C> Endpoint for Request<'_, C>
where
    C: client::Inner,
{
    const METHOD: reqwest::Method = reqwest::Method::GET;

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

impl<'a, C> Request<'a, C>
where
    C: client::Inner,
{
    pub(crate) fn new(client: &'a C, transaction_id: &'a str) -> Self {
        let endpoint = format!("/transactions/{transaction_id}");
        Self {
            client,
            endpoint,
            expand_merchant: false,
        }
    }

    /// Optionally expand the merchant field from an id string into a struct
    /// container merchant details
    pub const fn expand_merchant(mut self) -> Self {
        self.expand_merchant = true;
        self
    }

    /// Consume the request and return the [`Transaction`]
    pub async fn send(self) -> Result<Transaction> {
        self.client.handle_request(&self).await
    }
}
