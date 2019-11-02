//! Endpoints for retrieving and manipulating transactions

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

mod list;
pub use list::ListTransactions;

#[derive(Deserialize, Debug)]
struct Transactions {
    transactions: Vec<Transaction>,
}

impl From<Transactions> for Vec<Transaction> {
    fn from(transactions: Transactions) -> Self {
        transactions.transactions
    }
}

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
#[serde(rename_all = "snake_case")]
pub enum Category {
    General,
    EatingOut,
    Expenses,
    Transport,
    Cash,
    Bills,
    Entertainment,
    Shopping,
    Holidays,
    Groceries,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MerchantInfo {
    Id(String),
    Details(Merchant),
}

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
