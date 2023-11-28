//! Accounts API endpoint

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A struct representing a Monzo Account
#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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
#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Owner {
    /// The user ID of the owner
    pub user_id: String,

    /// The preferred name of the owner
    pub preferred_name: String,

    /// The preferred first name of the owner
    pub preferred_first_name: String,
}

/// Types of monzo account
#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

pub(crate) use list::Request as List;
mod list {

    use crate::endpoints::Endpoint;

    /// An object representing a request to the Monzo API for a list of accounts
    pub struct Request;

    impl Endpoint for Request {
        const METHOD: reqwest::Method = reqwest::Method::GET;

        fn endpoint(&self) -> &str {
            "/accounts"
        }
    }
}
