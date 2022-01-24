use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Pagination, Since, Transaction};
use crate::{
    client::{self, handle_request},
    endpoints::Endpoint,
    Result,
};

/// A request to retrieve a list of transactions from the Monzo API
///
/// Use the builder-style methods to set optional fields on the request
#[derive(Debug)]
#[must_use]
pub struct Request<'a> {
    client: &'a dyn client::Inner,
    query: Query<'a>,
}

impl<'a> Endpoint for Request<'a> {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> &str {
        "/transactions"
    }

    fn query(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.query)
    }
}

impl<'a> Request<'a> {
    pub(crate) fn new(client: &'a dyn client::Inner, account_id: &'a str) -> Self {
        let query = Query {
            account_id,
            pagination: Pagination::default(),
            expand_merchant: None,
        };

        Self { client, query }
    }

    /// Only return transactions which occurred after the given `DateTime`
    pub fn since(mut self, datetime: DateTime<Utc>) -> Self {
        self.query.pagination.since = Some(Since::Timestamp(datetime));
        self
    }

    /// Only return transactions which occurred after the given transaction.
    ///
    /// This can be used for paginating.
    pub fn since_transaction(mut self, transaction_id: String) -> Self {
        self.query.pagination.since = Some(Since::ObjectId(transaction_id));
        self
    }

    /// Only return transactions which occurred before a given `DateTime`
    pub const fn before(mut self, datetime: DateTime<Utc>) -> Self {
        self.query.pagination.before = Some(datetime);
        self
    }

    /// Set the maximum number of transactions to be returned
    pub const fn limit(mut self, limit: u16) -> Self {
        self.query.pagination.limit = Some(limit);
        self
    }

    /// Optionally expand the merchant field from an id string into a struct
    /// container merchant details
    pub const fn expand_merchant(mut self) -> Self {
        self.query.expand_merchant = Some("merchant");
        self
    }

    /// Consume the request and return the list of [`Transaction`]s
    pub async fn send(self) -> Result<Vec<Transaction>> {
        #[derive(Deserialize)]
        struct Response {
            transactions: Vec<Transaction>,
        }

        let response: Response = handle_request(self.client, &self).await?;

        Ok(response.transactions)
    }
}

#[derive(Serialize, Debug)]
struct Query<'a> {
    account_id: &'a str,

    #[serde(flatten)]
    pagination: Pagination,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expand[]")]
    expand_merchant: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    transactions: Vec<Transaction>,
}

impl From<Response> for Vec<Transaction> {
    fn from(response: Response) -> Self {
        response.transactions
    }
}
