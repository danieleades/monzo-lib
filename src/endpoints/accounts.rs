//! Accounts API endpoint

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A struct representing a Monzo Account
#[derive(Deserialize, Debug)]
pub struct Account {

    id: String,

    closed: bool,

    created: DateTime<Utc>,

    description: String,

    r#type: Type,

    currency: String,

    country_code: String,

    // TODO: this can be an enum. either its a normal account and there's one owner, or its a joint account and there's two
    owners: Vec<Owner>,

    account_number: String,

    sort_code: String,
}

impl Account {
    /// The unique ID of the account
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Whether the account has been closed
    pub fn closed(&self) -> bool {
        self.closed
    }

    /// The DateTime that the account was created
    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    /// The account description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// The type of the account
    pub fn account_type(&self) -> &Type {
        &self.r#type
    }

    /// This the a three-letter currency code
    pub fn currency(&self) -> &String {
        &self.currency
    }

    /// This is a country code for the country where the account is held
    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    /// A vector of account owners
    pub fn owners(&self) -> &Vec<Owner> {
        &self.owners
    }

    /// The account number
    pub fn account_number(&self) -> &String {
        &self.account_number
    }

    /// The sort code
    pub fn sort_code(&self) -> &String {
        &self.sort_code
    }
}

/// Struct representating an owner of a Monzo account
#[derive(Deserialize, Debug)]
pub struct Owner {
    user_id: String,
    preferred_name: String,
    preferred_first_name: String,
}

/// Types of monzo account
#[derive(Deserialize, Debug)]
#[serde(rename="snake_case")]
#[serde(untagged)]
#[non_exhaustive]
pub enum Type {
    /// A standard monzo account
    UkRetail,

    /// A monzo joint account
    UkRetailJoint,
}

pub use list::Request as List;
mod list {

    use super::Account;
    use crate::{endpoints::handle_response, Result};
    use serde::Deserialize;

    /// A struct representing a collection of accounts
    #[derive(Deserialize, Debug)]
    struct Accounts {
        accounts: Vec<Account>,
    }

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

        /// Consume the request and return a future that will resolve to a list
        /// of accounts
        pub async fn send(self) -> Result<Vec<Account>> {
            handle_response(self.request_builder)
                .await
                .map(|accounts: Accounts| accounts.accounts)
        }
    }
}
