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

    owners: Vec<Owner>,

    business_id: Option<String>,

    account_number: String,

    sort_code: String,
}

impl Account {
    /// The unique ID of the account
    #[must_use]
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Whether the account has been closed
    #[must_use]
    pub fn closed(&self) -> bool {
        self.closed
    }

    /// The `DateTime` that the account was created
    #[must_use]
    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    /// The account description
    #[must_use]
    pub fn description(&self) -> &String {
        &self.description
    }

    //     /// The type of the account
    // #[must_use]
    // pub fn account_type(&self) -> &Type {
    // &self.r#type
    // }

    /// This the a three-letter currency code
    #[must_use]
    pub fn currency(&self) -> &String {
        &self.currency
    }

    /// This is a country code for the country where the account is held
    #[must_use]
    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    /// A vector of account owners
    #[must_use]
    pub fn owners(&self) -> &Vec<Owner> {
        &self.owners
    }

    /// The business ID
    #[must_use]
    pub fn business_id(&self) -> &Option<String> {
        &self.business_id
    }

    /// The account number
    #[must_use]
    pub fn account_number(&self) -> &String {
        &self.account_number
    }

    /// The sort code
    #[must_use]
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
#[serde(rename_all = "snake_case")]
#[allow(clippy::pub_enum_variant_names)]
pub enum Type {
    /// A standard monzo account
    UkRetail,

    /// A monzo joint account
    UkRetailJoint,

    /// A monzo business account
    UkBusiness,
}

pub use list::Request as List;
mod list {

    use super::Account;
    use crate::{endpoints::handle_response, Result};
    use serde::Deserialize;

    /// A struct representing a collection of accounts
    #[derive(Deserialize, Debug)]
    pub(super) struct Accounts {
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

#[cfg(test)]
mod tests {
    use super::list::Accounts;

    #[test]
    fn deserialise_account() {
        let raw_account_string = r#"
        {
            "accounts": [
                {
                    "id": "acc_XXXX",
                    "closed": false,
                    "created": "2019-06-12T17:44:35.266Z",
                    "description": "user_XXXX",
                    "type": "uk_retail",
                    "currency": "GBP",
                    "country_code": "GB",
                    "owners": [
                        {
                            "user_id": "user_XXXX",
                            "preferred_name": "Daniel Eades",
                            "preferred_first_name": "Daniel"
                        }
                    ],
                    "account_number": "12345678",
                    "sort_code": "040004",
                    "payment_details": {
                        "locale_uk": {
                            "account_number": "12345678",
                            "sort_code": "040004"
                        }
                    }
                },
                {
                    "id": "acc_XXXX",
                    "closed": false,
                    "created": "2019-08-01T13:46:10.041Z",
                    "description": "Joint account between user_XXXX and user_YYYY",
                    "type": "uk_retail_joint",
                    "currency": "GBP",
                    "country_code": "GB",
                    "owners": [
                        {
                            "user_id": "user_XXXX",
                            "preferred_name": "Daniel Eades",
                            "preferred_first_name": "Daniel"
                        },
                        {
                            "user_id": "user_YYYY",
                            "preferred_name": "Holly Johnstone",
                            "preferred_first_name": "Holly"
                        }
                    ],
                    "account_number": "87654321",
                    "sort_code": "040004",
                    "payment_details": {
                        "locale_uk": {
                            "account_number": "87654321",
                            "sort_code": "040004"
                        }
                    }
                },
                {
                    "id": "acc_XXXX",
                    "closed": false,
                    "created": "2019-06-12T17:44:35.266Z",
                    "description": "Business Name",
                    "type": "uk_business",
                    "currency": "GBP",
                    "country_code": "GB",
                    "owners": [
                        {
                            "user_id": "user_XXXX",
                            "preferred_name": "Daniel Eades",
                            "preferred_first_name": "Daniel"
                        }
                    ],
                    "business_id": "business_XXXX",
                    "account_number": "12345678",
                    "sort_code": "040004",
                    "payment_details": {
                        "locale_uk": {
                            "account_number": "12345678",
                            "sort_code": "040004"
                        }
                    }
                }
            ]
        }"#;

        serde_json::from_str::<Accounts>(&raw_account_string).unwrap();
    }
}
