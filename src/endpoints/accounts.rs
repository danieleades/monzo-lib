use super::RequestBuilder;
use chrono::{DateTime, Utc};
use serde::Deserialize;

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
pub struct AccountsRequest {}

pub type AccountsRequestBuilder = RequestBuilder<AccountsRequest, Accounts>;