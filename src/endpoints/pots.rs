//! Pots endpoints

use chrono::{DateTime, Utc};
use serde::Deserialize;

mod list;
pub use list::ListPots;

mod deposit;
pub use deposit::DepositIntoPot;

/// A collection of Monzo pots
#[derive(Deserialize, Debug)]
pub struct Pots {
    pots: Vec<Pot>,
}

/// Representation of a Monzo pot
#[derive(Deserialize, Debug)]
pub struct Pot {
    /// Unique ID for this Monzo pot
    id: String,

    /// The human-readable name for this pot
    name: String,

    /// A reference to the built in Monzo image for this pot (may be an empty
    /// string)
    style: String,

    /// The pot balance, in the minor units of the specified currency
    balance: i64,

    /// Three letter code for the pot's currency
    currency: String,

    /// The datetime that the pot was created
    created: DateTime<Utc>,

    /// The datetime that the pot was last modified
    updated: DateTime<Utc>,

    /// true if the pot has been deleted
    ///
    /// *Note that in future the API will simply not return pots which have been
    /// deleted*
    deleted: bool,
}

impl Pot {
    /// Unique ID for this Monzo pot
    pub fn id(&self) -> &String {
        &self.id
    }

    /// The human-readable name for this pot
    pub fn name(&self) -> &String {
        &self.name
    }

    /// A reference to the built in Monzo image for this pot (may be an empty
    /// string)
    pub fn style(&self) -> &String {
        &self.style
    }

    /// The pot balance, in the minor units of the specified currency
    pub fn balance(&self) -> i64 {
        self.balance
    }

    /// Three letter code for the pot's currency
    pub fn currency(&self) -> &String {
        &self.currency
    }

    /// The datetime that the pot was created
    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    /// The datetime that the pot was last modified
    pub fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }

    /// true if the pot has been deleted
    ///
    /// *Note that in future the API will simply not return pots which have been
    /// deleted*
    pub fn deleted(&self) -> bool {
        self.deleted
    }
}
