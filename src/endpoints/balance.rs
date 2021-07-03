//! Acount balance

use serde::Deserialize;

/// The balance of a Monzo Account
#[derive(Deserialize, Debug)]
#[must_use]
pub struct Balance {
    /// The account balance, in the minor units of the listed currency. ie for
    /// GBP, the balance is in pence.
    pub balance: i64,

    /// The total account balance. I haven't figured out what the difference is
    /// yet
    pub total_balance: i64,

    /// three-letter currency code for the account
    pub currency: String,

    /// total expenditure so far this calendar day
    pub spend_today: i64,
}

pub(crate) use get::Request as Get;
mod get {
    use super::Balance;
    use crate::endpoints::{Endpoint, Resolve};
    use serde::Serialize;

    /// An object representing a request to the Monzo API for a list of accounts
    pub struct Request<'a> {
        query: Query<'a>,
    }

    impl<'a> Request<'a> {
        pub(crate) fn new(account_id: &'a str) -> Self {
            let query = Query { account_id };
            Self { query }
        }
    }

    impl<'a> Endpoint for Request<'a> {
        fn method(&self) -> http::Method {
            http::Method::GET
        }

        fn endpoint(&self) -> &str {
            "https://api.monzo.com/balance"
        }

        fn query(&self) -> Option<&dyn erased_serde::Serialize> {
            Some(&self.query)
        }
    }

    #[derive(Debug, Serialize)]
    struct Query<'a> {
        account_id: &'a str,
    }

    impl<'a> Resolve for Request<'a> {
        type Response = Balance;

        fn resolve(&self, bytes: &[u8]) -> serde_json::Result<Self::Response> {
            serde_json::from_slice(bytes)
        }
    }
}
