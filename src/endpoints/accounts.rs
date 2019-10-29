//! Accounts API endpoint

use crate::{endpoints::handle_response, into_future::IntoFuture, Result};
use chrono::{DateTime, Utc};
use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::future::Future;

/// A struct representing a collection of accounts
#[derive(Deserialize, Debug)]
pub struct Accounts {
    accounts: Vec<Account>,
}

impl std::ops::Deref for Accounts {
    type Target = Vec<Account>;
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}

/// A struct representing a Monzo Account
#[derive(Deserialize, Debug)]
pub struct Account {
    /// the unique ID of the accounts
    pub id: String,

    /// account description
    pub description: String,

    /// the date-time that the account was created
    pub created: DateTime<Utc>,
}

/// An object representing a request to the Monzo API for a list of accounts
pub struct ListAccounts {
    request_builder: reqwest::RequestBuilder,
}

impl ListAccounts {
    pub(crate) fn new(http_client: &HttpClient, access_token: impl AsRef<str>) -> Self {
        let request_builder = http_client
            .get("https://api.monzo.com/accounts")
            .bearer_auth(access_token.as_ref());

        Self { request_builder }
    }

    /// Consume the request and return a future that will resolve to a list of accounts
    pub async fn send(self) -> Result<Accounts> {
        handle_response(self.request_builder).await
    }
}

impl IntoFuture for ListAccounts {
    type Output = Result<Accounts>;
    type Future = impl Future<Output = Self::Output>;

    fn into_future(self) -> Self::Future {
        self.send()
    }
}
