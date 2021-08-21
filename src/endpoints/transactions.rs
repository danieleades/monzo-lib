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
    pub category: String,

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

    /// This can be either None, the merchant ID, or an object containing the
    /// merchant details
    pub merchant: Option<MerchantInfo>,

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

    /// Strong Customer Authentication blocking 'not present' transaction
    ScaNotAuthenticatedCardNotPresent,

    /// Requires SCA
    StrongCustomerAuthenticationRequired,

    /// All other errors
    Other,
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
    pub category: String,
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

#[cfg(test)]
mod tests {
    #![allow(clippy::too_many_lines, clippy::non_ascii_literal)]
    use super::Transaction;

    #[test]
    fn deserialise_expanded_transaction() {
        let raw = r##"
        {
          "id": "tx_0000A1aBC2Dbc34Ede5fEH",
          "created": "2021-06-29T13:10:09.992Z",
          "description": "Online Subscription",
          "amount": -5000,
          "fees": {},
          "currency": "GBP",
          "merchant": {
            "id": "merch_000000abcABCDEFGdHIeJ0",
            "group_id": "grp_000000abc1ABde2fChDE34",
            "created": "2016-01-08T00:20:13.969Z",
            "name": "Online Service",
            "logo": "https://mondo-logo-cache.appspot.com/twitter/ServiceUk/?size=large",
            "emoji": "💻",
            "category": "entertainment",
            "online": true,
            "atm": false,
            "address": {
              "short_formatted": "Somewhere in the world",
              "formatted": "world",
              "address": "",
              "city": "",
              "region": "",
              "country": "GLO",
              "postcode": "",
              "latitude": 50.99999999999999,
              "longitude": 5.111111111111111,
              "zoom_level": 5,
              "approximate": true
            },
            "updated": "2021-06-17T14:21:38.608Z",
            "metadata": {
              "created_for_merchant": "merch_000000abcABCDEFGdHIeJ0",
              "created_for_transaction": "tx_0000A1aBC2Dbc34Ede5fEH",
              "provider": "user",
              "provider_id": "",
              "suggested_tags": "#subscription #personal",
              "twitter_id": "ServiceUk",
              "website": "service.co.uk"
            },
            "disable_feedback": false
          },
          "notes": "Subscription to online service",
          "metadata": {
            "ledger_insertion_id": "entryset_0000A2bBcDEF3HdIJK4LMe",
            "mastercard_approval_type": "full",
            "mastercard_auth_message_id": "mcauthmsg_0000A2bBcDEF3HdIJK4LMe",
            "mastercard_card_id": "mccard_0000A2bBcDEF3HdIJK4LMe",
            "mastercard_lifecycle_id": "mclifecycle_0000A2bBcDEF3HdIJK4LMe",
            "mcc": "1234"
          },
          "labels": null,
          "attachments": null,
          "international": null,
          "category": "bills",
          "categories": {
            "bills": -5000
          },
          "is_load": false,
          "settled": "2021-06-30T00:46:44.233Z",
          "local_amount": -3900,
          "local_currency": "GBP",
          "updated": "2021-06-30T00:46:44.589Z",
          "account_id": "acc_99999aAbBc0DEFH1I2JdKL",
          "user_id": "user_000000abcABCDEFGdHIeJ",
          "counterparty": {},
          "scheme": "mastercard",
          "dedupe_id": "mclifecycle",
          "originator": false,
          "include_in_spending": true,
          "can_be_excluded_from_breakdown": true,
          "can_be_made_subscription": true,
          "can_split_the_bill": true,
          "can_add_to_tab": true,
          "amount_is_pending": false,
          "atm_fees_detailed": null
        }
        "##;

        serde_json::from_str::<Transaction>(raw).expect("couldn't decode Transaction from json");
    }

    #[test]
    fn deserialise_declined_transaction() {
        let raw = r##"
        {
          "id": "tx_0000A1aBC2Dbc34Ede5fEH",
          "created": "2021-06-29T13:10:09.992Z",
          "description": "Online Subscription",
          "amount": -5000,
          "fees": {},
          "currency": "GBP",
          "merchant": {
            "id": "merch_000000abcABCDEFGdHIeJ0",
            "group_id": "grp_000000abc1ABde2fChDE34",
            "created": "2016-01-08T00:20:13.969Z",
            "name": "Online Service",
            "logo": "https://mondo-logo-cache.appspot.com/twitter/ServiceUk/?size=large",
            "emoji": "💻",
            "category": "entertainment",
            "online": true,
            "atm": false,
            "address": {
              "short_formatted": "Somewhere in the world",
              "formatted": "world",
              "address": "",
              "city": "",
              "region": "",
              "country": "GLO",
              "postcode": "",
              "latitude": 50.99999999999999,
              "longitude": 5.111111111111111,
              "zoom_level": 5,
              "approximate": true
            },
            "updated": "2021-06-17T14:21:38.608Z",
            "metadata": {
              "created_for_merchant": "merch_000000abcABCDEFGdHIeJ0",
              "created_for_transaction": "tx_0000A1aBC2Dbc34Ede5fEH",
              "provider": "user",
              "provider_id": "",
              "suggested_tags": "#subscription #personal",
              "twitter_id": "ServiceUk",
              "website": "service.co.uk"
            },
            "disable_feedback": false
          },
          "notes": "Subscription to online service",
          "metadata": {
            "ledger_insertion_id": "entryset_0000A2bBcDEF3HdIJK4LMe",
            "mastercard_approval_type": "full",
            "mastercard_auth_message_id": "mcauthmsg_0000A2bBcDEF3HdIJK4LMe",
            "mastercard_card_id": "mccard_0000A2bBcDEF3HdIJK4LMe",
            "mastercard_lifecycle_id": "mclifecycle_0000A2bBcDEF3HdIJK4LMe",
            "mcc": "1234"
          },
          "labels": null,
          "attachments": null,
          "international": null,
          "category": "bills",
          "categories": {
            "bills": -5000
          },
          "is_load": false,
          "settled": "2021-06-30T00:46:44.233Z",
          "decline_reason": "SCA_NOT_AUTHENTICATED_CARD_NOT_PRESENT",
          "local_amount": -3900,
          "local_currency": "GBP",
          "updated": "2021-06-30T00:46:44.589Z",
          "account_id": "acc_99999aAbBc0DEFH1I2JdKL",
          "user_id": "user_000000abcABCDEFGdHIeJ",
          "counterparty": {},
          "scheme": "mastercard",
          "dedupe_id": "mclifecycle",
          "originator": false,
          "include_in_spending": true,
          "can_be_excluded_from_breakdown": true,
          "can_be_made_subscription": true,
          "can_split_the_bill": true,
          "can_add_to_tab": true,
          "amount_is_pending": false,
          "atm_fees_detailed": null
        }
        "##;

        serde_json::from_str::<Transaction>(raw).expect("couldn't decode Transaction from json");
    }

    #[test]
    // Tests for null merchant
    fn deserialise_topup_transaction() {
        let raw = r#"
        {
          "id": "tx_0000A1aBC2Dbc34Ede5fEF",
          "created": "2021-07-01T00:21:30.935Z",
          "description": "USER",
          "amount": 2000,
          "fees": {},
          "currency": "GBP",
          "merchant": null,
          "notes": "USER",
          "metadata": {
            "faster_payment": "true",
            "fps_fpid": "FP123456789123456789123456789123456",
            "fps_payment_id": "FP123456789123456789123456789123456",
            "insertion": "entryset_0000A1aBC2Dbc34Ede5fEF",
            "notes": "USER",
            "trn": "FP12345678912345"
          },
          "labels": null,
          "attachments": null,
          "international": null,
          "category": "general",
          "categories": null,
          "is_load": false,
          "settled": "2021-07-01T06:00:00Z",
          "local_amount": 2000,
          "local_currency": "GBP",
          "updated": "2021-07-01T00:21:31.022Z",
          "account_id": "acc_99999aAbBc0DEFH1I2JdKL",
          "user_id": "",
          "counterparty": {
            "account_number": "12345678",
            "name": "John Smith",
            "sort_code": "987654",
            "user_id": "anonuser_1234567a89b123456cd7e8"
          },
          "scheme": "payport_faster_pajments",
          "dedupe_id": "com.monzo.fps:1234:FP123456789123456789123456789123456:INBOUND",
          "originator": false,
          "include_in_spending": false,
          "can_be_excluded_from_breakdown": false,
          "can_be_made_subscription": false,
          "can_split_the_bill": false,
          "can_add_to_tab": false,
          "amount_is_pending": false,
          "atm_fees_detailed": null
        }
        "#;

        serde_json::from_str::<Transaction>(raw).expect("couldn't decode Transaction from json");
    }

    #[test]
    fn deserialise_list() {
        use serde::Deserialize;
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct Response {
            transactions: Vec<Transaction>,
        }

        let raw = r##"
        {
          "transactions": [
            {
              "id": "tx_0000A1aBC2Dbc34Ede5fEH",
              "created": "2021-06-29T13:10:09.992Z",
              "description": "Online Subscription",
              "amount": -3900,
              "fees": {},
              "currency": "GBP",
              "merchant": {
                "id": "merch_000000abcABCDEFGdHIeJ0",
                "group_id": "grp_000000abc1ABde2fChDE34",
                "created": "2016-01-08T00:20:13.969Z",
                "name": "Online Service",
                "logo": "https://mondo-logo-cache.appspot.com/twitter/ServiceUk/?size=large",
                "emoji": "💻",
                "category": "entertainment",
                "online": true,
                "atm": false,
                "address": {
                  "short_formatted": "Somewhere in the world",
                  "formatted": "world",
                  "address": "",
                  "city": "",
                  "region": "",
                  "country": "GLO",
                  "postcode": "",
                  "latitude": 50.99999999999999,
                  "longitude": 5.111111111111111,
                  "zoom_level": 5,
                  "approximate": true
                },
                "updated": "2021-06-17T14:21:38.608Z",
                "metadata": {
                  "created_for_merchant": "merch_000000abcABCDEFGdHIeJ0",
                  "created_for_transaction": "tx_0000A1aBC2Dbc34Ede5fEH",
                  "provider": "user",
                  "provider_id": "",
                  "suggested_tags": "#subscription #personal",
                  "twitter_id": "ServiceUk",
                  "website": "service.co.uk"
                },
                "disable_feedback": false
              },
              "notes": "Subscription to online service",
              "metadata": {
                "ledger_insertion_id": "entryset_0000A2bBcDEF3HdIJK4LMe",
                "mastercard_approval_type": "full",
                "mastercard_auth_message_id": "mcauthmsg_0000A2bBcDEF3HdIJK4LMe",
                "mastercard_card_id": "mccard_0000A2bBcDEF3HdIJK4LMe",
                "mastercard_lifecycle_id": "mclifecycle_0000A2bBcDEF3HdIJK4LMe",
                "mcc": "1234"
              },
              "labels": null,
              "attachments": null,
              "international": null,
              "category": "bills",
              "categories": {
                "bills": -3900
              },
              "is_load": false,
              "settled": "2021-06-30T00:46:44.233Z",
              "local_amount": -3900,
              "local_currency": "GBP",
              "updated": "2021-06-30T00:46:44.589Z",
              "account_id": "acc_99999aAbBc0DEFH1I2JdKL",
              "user_id": "user_000000abcABCDEFGdHIeJ",
              "counterparty": {},
              "scheme": "mastercard",
              "dedupe_id": "mclifecycle",
              "originator": false,
              "include_in_spending": true,
              "can_be_excluded_from_breakdown": true,
              "can_be_made_subscription": true,
              "can_split_the_bill": true,
              "can_add_to_tab": true,
              "amount_is_pending": false,
              "atm_fees_detailed": null
            },
            {
              "id": "tx_0000A1aBC2Dbc34Ede5fEF",
              "created": "2021-07-01T00:21:30.935Z",
              "description": "USER",
              "amount": 2000,
              "fees": {},
              "currency": "GBP",
              "merchant": null,
              "notes": "USER",
              "metadata": {
                "faster_payment": "true",
                "fps_fpid": "FP123456789123456789123456789123456",
                "fps_payment_id": "FP123456789123456789123456789123456",
                "insertion": "entryset_0000A1aBC2Dbc34Ede5fEF",
                "notes": "USER",
                "trn": "FP12345678912345"
              },
              "labels": null,
              "attachments": null,
              "international": null,
              "category": "general",
              "categories": null,
              "is_load": false,
              "settled": "2021-07-01T06:00:00Z",
              "local_amount": 2000,
              "local_currency": "GBP",
              "updated": "2021-07-01T00:21:31.022Z",
              "account_id": "acc_99999aAbBc0DEFH1I2JdKL",
              "user_id": "",
              "counterparty": {
                "account_number": "12345678",
                "name": "John Smith",
                "sort_code": "987654",
                "user_id": "anonuser_1234567a89b123456cd7e8"
              },
              "scheme": "payport_faster_pajments",
              "dedupe_id": "com.monzo.fps:1234:FP123456789123456789123456789123456:INBOUND",
              "originator": false,
              "include_in_spending": false,
              "can_be_excluded_from_breakdown": false,
              "can_be_made_subscription": false,
              "can_split_the_bill": false,
              "can_add_to_tab": false,
              "amount_is_pending": false,
              "atm_fees_detailed": null
            }
          ]
        }
        "##;

        serde_json::from_str::<Response>(raw).expect("couldn't decode Transaction from json");
    }
}
