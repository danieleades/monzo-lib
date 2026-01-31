//! Accounts API endpoint

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A struct representing a Monzo Account
#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[non_exhaustive]
pub struct Account {
    /// The unique ID of the account
    pub id: String,

    /// The account details including type and banking information
    #[serde(flatten)]
    pub account_type: Type,

    /// Whether the account has been closed
    pub closed: bool,

    /// The `DateTime` that the account was created
    pub created: DateTime<Utc>,

    /// The account description
    pub description: String,

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

/// Account details including type and banking information
#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Type {
    /// A standard monzo account
    UkRetail(AccountDetails),

    /// A monzo joint account
    UkRetailJoint(AccountDetails),

    /// A monzo business account
    UkBusiness(AccountDetails),

    /// A monzo rewards account
    UkRewards,

    /// A monzo flex account
    UkMonzoFlex,

    /// A monzo loan account
    UkLoan,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
/// Banking information
pub struct AccountDetails {
    /// The account number
    pub account_number: String,
    /// The sort code
    pub sort_code: String,
}

pub(crate) use list::Request as List;
mod list {

    use crate::endpoints::Endpoint;

    /// An object representing a request to the Monzo API for a list of accounts
    pub struct Request;

    impl Endpoint for Request {
        const METHOD: reqwest::Method = reqwest::Method::GET;

        fn endpoint(&self) -> &'static str {
            "/accounts"
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::Account;

    #[test_case(
        r#"
            {
      "id": "acc_00009",
      "closed": false,
      "created": "2021-06-12T00:00:00.000Z",
      "description": "user_00009",
      "type": "uk_retail",
      "owner_type": "personal",
      "is_flex": false,
      "product_type": "standard",
      "closed_account_app_access": false,
      "currency": "GBP",
      "legal_entity": "monzo_uk",
      "country_code": "GB",
      "country_code_alpha3": "GBR",
      "owners": [
        {
          "user_id": "user_0000",
          "preferred_name": "First Last",
          "preferred_first_name": "First"
        }
      ],
      "account_number": "12345678",
      "sort_code": "040004",
      "payment_details": {
        "locale_uk": {
          "account_number": "12345678",
          "sort_code": "040004"
        },
        "iban": {
          "unformatted": "GB90MONZXXXXXXXXXXX",
          "formatted": "GB90MONZXXXXXXXXXXXXX",
          "bic": "MONZGB2L",
          "usage_description": "Receive international payments in over 40 currencies. We charge a currency conversion fee. [Learn more...](monzo://backend_screen?id=international-payment-views:bank-transfer-info&instance_params_id=acc_00009jmHyLkxAPVUSe8H45)",
          "usage_description_web": "Receive international payments in over 40 currencies. We charge a currency conversion fee."
        }
      },
      "monzo_branch_address_formatted": "Monzo Bank, Broadwalk House, 5 Appold St, London EC2A 2AG, United Kingdom",
      "assets": {
        "image_url": "https://public-images.monzo.com/card_styles/account_icon/personal@3x.png"
      }
    }
        "#
        ; "uk_retail"
    )]
    #[test_case(
        r#"{
            "id": "acc_ID",
            "closed": false,
            "created": "2024-01-20T00:00:00.000Z",
            "description": "rewardsoptin_0000",
            "type": "uk_rewards",
            "owner_type": "personal",
            "is_flex": false,
            "product_type": "rewards",
            "closed_account_app_access": false,
            "currency": "GBP",
            "legal_entity": "monzo_uk",
            "country_code": "GB",
            "country_code_alpha3": "GBR",
            "owners": [
                {
                    "user_id": "user_0000",
                    "preferred_name": "First Last",
                    "preferred_first_name": "First"
                }
            ]
        }"#
        ; "uk_rewards"
    )]
    #[test_case(
        r#"{
            "id": "acc_0000",
            "closed": false,
            "created": "2024-01-01T00:00:00.000Z",
            "description": "monzoflex_0000",
            "type": "uk_monzo_flex",
            "owner_type": "personal",
            "is_flex": true,
            "product_type": "flex",
            "closed_account_app_access": false,
            "currency": "GBP",
            "legal_entity": "monzo_uk",
            "country_code": "GB",
            "country_code_alpha3": "GBR",
            "owners": [
                {
                    "user_id": "user_0000",
                    "preferred_name": "First Last",
                    "preferred_first_name": "First"
                }
            ],
            "assets": {
                "image_url": "https://public-images.monzo.com/card_styles/account_icon/flex@3x.png"
            }
        }"#
        ; "uk_monzo_flex"
    )]
    #[test_case(
        r#"{
            "id": "acc_0000",
            "closed": false,
            "created": "2024-01-01T00:00:00.000Z",
            "description": "loan_0000",
            "type": "uk_loan",
            "owner_type": "unknown",
            "is_flex": false,
            "product_type": "loan",
            "closed_account_app_access": false,
            "currency": "GBP",
            "legal_entity": "monzo_uk",
            "country_code": "GB",
            "country_code_alpha3": "GBR",
            "owners": [
                {
                    "user_id": "user_0000",
                    "preferred_name": "First Last",
                    "preferred_first_name": "First"
                }
            ]
        }"#
        ; "uk_loan"
    )]
    fn parse_account(json_data: &str) {
        let _account: Account = serde_json::from_str(json_data).unwrap();
    }
}
