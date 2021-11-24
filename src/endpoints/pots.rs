//! endpoints for working with Monzo pots

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::endpoints::utils::empty_string_as_none;

mod list;
pub(crate) use list::Request as List;
mod deposit;
pub(crate) use deposit::Request as Deposit;
mod withdraw;
pub(crate) use withdraw::Request as Withdraw;

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

    /// The goal balance for this pot, if set
    #[serde(default)]
    pub goal_amount: Option<i64>,

    /// The unique ID of the account associated with this pot
    pub current_account_id: String,

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

#[cfg(test)]
mod tests {

    use super::Pot;

    #[test]
    fn deserialise() {
        let raw = r#"
        {
            "id": "pot_1234",
            "name": "Savings",
            "style": "teal",
            "balance": 10,
            "currency": "GBP",
            "goal_amount": 1000000,
            "type": "flexible_savings",
            "product_id": "XXX",
            "current_account_id": "acc_1234",
            "cover_image_url": "",
            "isa_wrapper": "ISA",
            "round_up": false,
            "round_up_multiplier": null,
            "is_tax_pot": false,
            "created": "2019-04-28T06:36:54.318Z",
            "updated": "2019-05-11T00:31:04.256Z",
            "deleted": false,
            "locked": false,
            "charity_id": "",
            "available_for_bills": false
        }
        "#;

        serde_json::from_str::<Pot>(raw).expect("couldn't decode Pot from json");
    }
}
