use super::Transaction;
use crate::{endpoints::handle_response, Result};

/// A request to retrieve a list of transactions from the Monzo API
///
/// Use the builder-style methods to set optional fields on the request
pub struct RetrieveTransaction {
    reqwest_builder: reqwest::RequestBuilder,
    expand_merchant: bool,
}

impl RetrieveTransaction {
    pub(crate) fn new(
        http_client: &reqwest::Client,
        access_token: &str,
        transaction_id: &str,
    ) -> Self {
        let reqwest_builder = http_client
            .get(&format!(
                "https://api.monzo.com/transactions/{}",
                transaction_id
            ))
            .bearer_auth(access_token);

        Self {
            reqwest_builder,
            expand_merchant: false,
        }
    }

    /// Consume the request and return a future that resolves to a List of
    /// Transactions
    pub async fn send(self) -> Result<Transaction> {
        let mut reqwest_builder = self.reqwest_builder;
        if self.expand_merchant {
            reqwest_builder = reqwest_builder.query(&("expand[]", "merchant"))
        }
        handle_response(reqwest_builder).await
    }

    /// Optionally expand the merchant field from an id string into a struct
    /// container merchant details
    pub fn expand_merchant(mut self) -> Self {
        self.expand_merchant = true;
        self
    }
}
