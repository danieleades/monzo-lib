//! Accounts API endpoint

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A struct representing a Monzo Account
#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Account {
    /// The unique ID of the account
    pub id: String,

    /// Whether the account has been closed
    pub closed: bool,

    /// The `DateTime` that the account was created
    pub created: DateTime<Utc>,

    /// The account description
    pub description: String,

    /// The type of the account
    #[serde(rename = "type")]
    pub account_type: Type,

    /// This the a three-letter currency code
    pub currency: String,

    /// This is a country code for the country where the account is held
    pub country_code: String,

    /// A vector of account owners
    pub owners: Vec<Owner>,

    /// The business ID
    ///
    /// This is only set for business accounts
    pub business_id: Option<String>,

    /// The account number
    pub account_number: String,

    /// The sort code
    pub sort_code: String,
}

/// Struct representating an owner of a Monzo account
#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Owner {
    user_id: String,
    preferred_name: String,
    preferred_first_name: String,
}

/// Types of monzo account
#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
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
    use crate::endpoints::{Endpoint, Resolve};
    use serde::Deserialize;

    /// An object representing a request to the Monzo API for a list of accounts
    pub struct Request;

    impl Endpoint for Request {
        fn method(&self) -> http::Method {
            http::Method::GET
        }

        fn endpoint(&self) -> &str {
            "https://api.monzo.com/accounts"
        }
    }

    impl Resolve for Request {
        type Response = Vec<Account>;

        fn resolve(&self, bytes: &[u8]) -> serde_json::Result<Self::Response> {
            /// A struct representing a collection of accounts
            #[derive(Deserialize)]
            pub(crate) struct Accounts {
                accounts: Vec<Account>,
            }
            let accounts: Accounts = serde_json::from_slice(bytes)?;
            Ok(accounts.accounts)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::Resolve;

    #[test]
    fn deserialise() {
        let bytes = r#"
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
        }"#
        .as_bytes();

        super::list::Request.resolve(&bytes).unwrap();
    }
}
