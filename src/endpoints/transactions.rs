//! Retrieve and manipulate transactions

use crate::endpoints::utils::empty_string_as_none;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod list;
pub(crate) use list::Request as List;
mod get;
pub(crate) use get::Request as Get;

/// A Monzo transaction
#[allow(clippy::struct_excessive_bools)]
#[non_exhaustive]
#[derive(Deserialize, Debug)]
pub struct Transaction {
    /// The account balance after the transation
    pub account_balance: i64,

    /// The unique ID of the account associated with the transaction
    pub account_id: String,

    /// The amount of the transaction, in the smallest unit of currency (ie.
    /// 'pence' or 'cents')
    pub amount: i64,

    /// Whether the transaction is pending, or complete
    pub amount_is_pending: bool,

    can_add_to_tab: bool,

    can_be_excluded_from_breakdown: bool,

    can_be_made_subscription: bool,

    can_split_the_bill: bool,

    /// The transaction category
    pub category: Category,

    /// The timestamp when the transaction was created
    pub created: DateTime<Utc>,

    /// The three-letter currency string for the transaction
    pub currency: String,

    /// The transaction description
    pub description: String,

    /// The unique transaction ID
    pub id: String,

    include_in_spending: bool,

    /// This can be either the merchant ID, or an object containing the merchant
    /// details
    pub merchant: MerchantInfo,

    metadata: HashMap<String, String>,

    /// User-added transaction notes
    pub notes: String,

    /// If the transaction was declined, this enum will encode the reason
    pub decline_reason: Option<DeclineReason>,

    /// Top-ups to an account are represented as transactions with a positive
    /// amount and is_load = true. Other transactions such as refunds, reversals
    /// or chargebacks may have a positive amount but is_load = false
    pub is_load: bool,

    /// The timestamp at wich the transaction was settled
    ///
    /// This is `None` if the transaction is authorised, but not yet complete.
    #[serde(deserialize_with = "empty_string_as_none")]
    pub settled: Option<DateTime<Utc>>,
}

/// The set of reasons for which a monzo transaction may be declined
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
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
