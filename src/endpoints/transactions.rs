//! Endpoints for retrieving and manipulating transactions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod list;
pub use list::Request as List;
mod get;
pub use get::Request as Get;

/// A Monzo transaction
#[derive(Deserialize, Debug)]
pub struct Transaction {
    account_balance: i64,

    account_id: String,

    amount: i64,

    amount_is_pending: bool,

    can_add_to_tab: bool,

    can_be_excluded_from_breakdown: bool,

    can_be_made_subscription: bool,

    can_split_the_bill: bool,

    category: Category,

    created: DateTime<Utc>,

    currency: String,

    description: String,

    id: String,

    include_in_spending: bool,

    merchant: Option<MerchantInfo>,

    metadata: HashMap<String, String>,

    notes: String,

    decline_reason: Option<DeclineReason>,

    is_load: bool,

    #[serde(deserialize_with = "empty_string_as_none")]
    settled: Option<DateTime<Utc>>,
}

/// The set of reasons for which a monzo transaction may be declined
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeclineReason {
    /// Not enough funds in account to complete transaction
    InsufficientFunds,

    /// Monzo card is not active
    CardInactive,

    /// The monzo card has been blocked
    CardBlocked,

    /// Incorrect CVC code used
    InvalidCvc,

    /// All other errors
    Other,
}

/// The set of categories by which Monzo transactions and merchants can be
/// categorised
#[derive(Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Category {
    /// General expenses
    General,

    /// Restaurants, Cafes, etc
    EatingOut,

    /// Work-related expenses
    Expenses,

    /// Getting around
    Transport,

    /// Cash withdrawals
    Cash,

    /// Bills and regular expenses
    Bills,

    /// Fun and Entertainment
    Entertainment,

    /// Treat yourself
    Shopping,

    /// Holiday expenses
    Holidays,

    /// Food and household items
    Groceries,
}

/// Merchant information which might be returned in transactions data.
///
/// An id or a struct may be returned depending on whether the 'expand merchant'
/// flag is set in the transactions request.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MerchantInfo {
    /// A unique ID associated with a merchant
    Id(String),

    /// Extra merchant information which may optionally be requested
    Details(Box<Merchant>),
}

/// Merchant details
#[derive(Deserialize, Debug)]
pub struct Merchant {
    address: Address,
    created: DateTime<Utc>,
    group_id: String,
    id: String,
    logo: String,
    emoji: String,
    name: String,
    category: Category,
}

/// Address details
#[derive(Deserialize, Debug)]
pub struct Address {
    address: String,
    city: String,
    country: String,
    latitude: f32,
    longitude: f32,
    postcode: String,
    region: String,
}

use serde::de::IntoDeserializer;

// see https://github.com/serde-rs/serde/issues/1425#issuecomment-439729881
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

#[derive(Serialize, Default)]
struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<Since>,

    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<DateTime<Utc>>,
}

/// The 'since' paramater of a pagination request can be either a timestamp or
/// an object id
#[derive(Serialize)]
#[serde(untagged)]
pub enum Since {
    /// A timestamp
    Timestamp(DateTime<Utc>),

    /// An id of an object
    ObjectId(String),
}
