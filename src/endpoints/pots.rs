//! endpoints for working with Monzo pots

use crate::endpoints::utils::empty_string_as_none;
use chrono::{DateTime, Utc};
use serde::Deserialize;

mod list;
pub(crate) use list::Request as List;
mod deposit;
pub(crate) use deposit::Request as Deposit;

/// Representation of a Monzo pot
#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub struct Pot {
    /// Unique ID for this Monzo pot
    pub id: String,

    /// The human-readable name for this pot
    pub name: String,

    /// A reference to the built in Monzo image for this pot
    #[serde(deserialize_with = "empty_string_as_none")]
    pub style: Option<String>,

    /// The pot balance, in the minor units of the specified currency
    pub balance: i64,

    /// Three letter code for the pot's currency
    pub currency: String,

    /// The datetime that the pot was created
    pub created: DateTime<Utc>,

    /// The datetime that the pot was last modified
    pub updated: DateTime<Utc>,

    /// true if the pot has been deleted
    ///
    /// *Note that in future the API will simply not return pots which have been
    /// deleted*
    pub deleted: bool,
}
