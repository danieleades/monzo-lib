//! Accounts API endpoint

use chrono::{DateTime, Utc};
use serde::Deserialize;

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
