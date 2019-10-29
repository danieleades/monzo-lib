//! Balance endpoint

use crate::{endpoints::handle_response, into_future::IntoFuture, Result};
use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::{future::Future, pin::Pin};

/// The balance of a Monzo Account
#[derive(Deserialize, Debug)]
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

/// An object representing a request to the Monzo API for a list of accounts
pub struct GetBalance<'a> {
    reqwest_builder: reqwest::RequestBuilder,
    account_id: &'a str,
}

impl<'a> GetBalance<'a> {
    pub(crate) fn new(
        http_client: &HttpClient,
        access_token: impl AsRef<str>,
        account_id: &'a str,
    ) -> Self {
        let reqwest_builder = http_client
            .get("https://api.monzo.com/balance")
            .bearer_auth(access_token.as_ref());

        Self {
            reqwest_builder,
            account_id,
        }
    }

    /// Consume the request and return a future that will resolve to the balance of the given account
    pub async fn send(self) -> Result<Balance> {
        handle_response(
            self.reqwest_builder
                .query(&[("account_id", self.account_id)]),
        )
        .await
    }
}

impl<'a> IntoFuture for GetBalance<'a> {
    type Output = Result<Balance>;
    type Future = Pin<Box<dyn Future<Output = Self::Output> + 'a>>;

    fn into_future(self) -> Self::Future {
        Box::pin(self.send())
    }
}

/* impl<'a> IntoFuture for GetBalance<'a> {
    type Output = Result<Balance>;
    type Future = impl Future<Output = Self::Output>;

    fn into_future(self) -> Self::Future {
        self.send()
    }
} */
