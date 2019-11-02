//! Accounts API endpoint

use crate::{endpoints::handle_response, into_future::IntoFuture, Result};
use chrono::{DateTime, Utc};
use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::future::Future;

/// A struct representing a collection of accounts
#[derive(Deserialize, Debug)]
struct Accounts {
    accounts: Vec<Account>,
}

/// A struct representing a Monzo Account
#[derive(Deserialize, Debug)]
pub struct Account {
    /// the unique ID of the accounts
    pub id: String,

    closed: bool,

    /// the date-time that the account was created
    pub created: DateTime<Utc>,

    /// account description
    pub description: String,

    r#type: String,

    currency: String,

    country_code: String,

    owners: Vec<Owner>,

    account_number: String,

    sort_code: String,
}

#[derive(Deserialize, Debug)]
struct Owner {
    user_id: String,
    preferred_name: String,
    preferred_first_name: String,
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

    /// Consume the request and return a future that will resolve to a list of
    /// accounts
    pub async fn send(self) -> Result<Vec<Account>> {
        handle_response(self.request_builder)
            .await
            .map(|accounts: Accounts| accounts.accounts)
    }
}

/* impl IntoFuture for ListAccounts {
    type Output = Result<Vec<Account>>;
    type Future = impl Future<Output = Self::Output>;

    fn into_future(self) -> Self::Future {
        self.send()
    }
} */
