use super::Transaction;
use crate::{
    client,
    endpoints::{Endpoint, Resolve},
    request_builder::RequestBuilder,
};

/// A request to retrieve a list of transactions from the Monzo API
///
/// Use the builder-style methods to set optional fields on the request
pub struct Request {
    endpoint: String,
    expand_merchant: bool,
}

impl Endpoint for Request {
    fn method(&self) -> http::Method {
        http::Method::GET
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

impl Resolve for Request {
    type Response = Transaction;

    fn resolve(&self, bytes: &[u8]) -> serde_json::Result<Self::Response> {
        serde_json::from_slice(bytes)
    }
}

impl Request {
    pub(crate) fn new(transaction_id: &str) -> Self {
        let endpoint = format!("https://api.monzo.com/transactions/{}", transaction_id);
        Self {
            endpoint,
            expand_merchant: false,
        }
    }
}

impl<'a, M> RequestBuilder<'a, M, Request>
where
    M: client::Inner,
{
    /// Optionally expand the merchant field from an id string into a struct
    /// container merchant details
    pub fn expand_merchant(mut self) -> Self {
        self.endpoint_ref_mut().expand_merchant = true;
        self
    }
}
