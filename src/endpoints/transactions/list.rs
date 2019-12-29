use super::{Pagination, Since, Transaction};
use crate::{endpoints::handle_response, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::future::{Future, IntoFuture};

/// A request to retrieve a list of transactions from the Monzo API
///
/// Use the builder-style methods to set optional fields on the request
pub struct Request<'a> {
    reqwest_builder: reqwest::RequestBuilder,
    payload: Payload<'a>,
}

impl<'a> Request<'a> {
    pub(crate) fn new(
        http_client: &reqwest::Client,
        access_token: &str,
        account_id: &'a str,
    ) -> Self {
        let reqwest_builder = http_client
            .get("https://api.monzo.com/transactions")
            .bearer_auth(access_token);

        let payload = Payload {
            account_id,
            pagination: Pagination::default(),
            expand_merchant: None,
        };

        Self {
            reqwest_builder,
            payload,
        }
    }

    /// Consume the request and return a future that resolves to a List of
    /// Transactions
    async fn send(self) -> Result<Vec<Transaction>> {
        #[derive(Deserialize)]
        struct Response {
            transactions: Vec<Transaction>,
        }

        let Response { transactions } =
            handle_response(self.reqwest_builder.form(&self.payload)).await?;

        Ok(transactions)
    }

    /// Only return transactions which occurred after the given `DateTime`
    pub fn since(mut self, datetime: DateTime<Utc>) -> Self {
        self.payload.pagination.since = Some(Since::Timestamp(datetime));
        self
    }

    /// Only return transactions which occurred after the given transaction.
    ///
    /// This can be used for paginating.
    pub fn since_transaction(mut self, transaction_id: String) -> Self {
        self.payload.pagination.since = Some(Since::ObjectId(transaction_id));
        self
    }

    /// Only return transactions which occurred before a given `DateTime`
    pub fn before(mut self, datetime: DateTime<Utc>) -> Self {
        self.payload.pagination.before = Some(datetime);
        self
    }

    /// Set the maximum number of transactions to be returned
    pub fn limit(mut self, limit: u16) -> Self {
        self.payload.pagination.limit = Some(limit);
        self
    }

    /// Optionally expand the merchant field from an id string into a struct
    /// container merchant details
    pub fn expand_merchant(mut self) -> Self {
        self.payload.expand_merchant = Some("merchant");
        self
    }
}

#[derive(Serialize)]
struct Payload<'a> {
    account_id: &'a str,

    #[serde(flatten)]
    pagination: Pagination,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expand[]")]
    expand_merchant: Option<&'a str>,
}

impl<'a> IntoFuture for Request<'a> {
    type Output = Result<Vec<Transaction>>;
    type Future = impl Future<Output = Self::Output> + 'a;
    fn into_future(self) -> Self::Future {
        self.send()
    }
}
