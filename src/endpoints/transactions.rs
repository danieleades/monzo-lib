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
    /// The unique ID of the account associated with the transaction
    pub account_id: String,

    /// The amount of the transaction, in the smallest unit of currency (ie.
    /// 'pence' or 'cents')
    pub amount: i64,

    /// Whether the transaction is pending, or complete
    pub amount_is_pending: bool,

    /// Whether the transaction can be added to a tab
    pub can_add_to_tab: bool,

    /// Whether the transaction can be excluded from breakdown
    pub can_be_excluded_from_breakdown: bool,

    /// Whether the transaction can be made into a recurring subscription
    pub can_be_made_subscription: bool,

    /// Whether the transaction can be split
    pub can_split_the_bill: bool,

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

    /// Whether transaction is included in spending
    pub include_in_spending: bool,

    /// This can be either the merchant ID, or an object containing the merchant
    /// details
    pub merchant: MerchantInfo,

    /// Any custom metadata which has been added to the transaction
    pub metadata: HashMap<String, String>,

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
#[derive(Deserialize, Debug, Clone, Copy)]
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
#[derive(Deserialize, Debug, Clone, Copy)]
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
#[allow(missing_docs)]
pub struct Merchant {
    pub address: Address,
    pub created: DateTime<Utc>,
    pub group_id: String,
    pub id: String,
    pub logo: String,
    pub emoji: String,
    pub name: String,
    pub category: Category,
}

/// Address details
#[derive(Deserialize, Debug)]
#[allow(missing_docs)]
pub struct Address {
    pub address: String,
    pub city: String,
    pub country: String,
    pub latitude: f32,
    pub longitude: f32,
    pub postcode: String,
    pub region: String,
}

#[derive(Serialize, Default, Debug)]
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
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Since {
    /// A timestamp
    Timestamp(DateTime<Utc>),

    /// An id of an object
    ObjectId(String),
}
