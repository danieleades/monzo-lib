//! Monzo API clients

use crate::{
    endpoints::{accounts, balance, feed_items, pots, transactions, who_am_i, Endpoint},
    request_builder::RequestBuilder,
    Result,
};
use async_trait::async_trait;

pub mod inner;

/// A generic trait of any HTTP client which also stores and manages an access
/// token.
#[async_trait]
pub trait Inner: Send + Sync {
    /// Construct end send an HTTP request using the provided Endpoint with
    /// bearer token authentication.
    async fn execute(
        &self,
        endpoint: &dyn Endpoint,
        access_token: Option<&str>,
    ) -> reqwest::Result<reqwest::Response>;

    /// Construct end send an HTTP request using the provided Endpoint.
    async fn execute_authenticated(
        &self,
        endpoint: &dyn Endpoint,
    ) -> reqwest::Result<reqwest::Response> {
        self.execute(endpoint, Some(self.access_token())).await
    }

    /// Return a reference to the current access token
    fn access_token(&self) -> &String;

    /// Manually set the access token
    fn set_access_token(&mut self, access_token: String);
}

/// A Monzo API client
#[derive(Debug)]
pub struct Client<M>
where
    M: Inner,
{
    inner_client: M,
}

impl<M> Client<M>
where
    M: Inner,
{
    /// Return a reference to the current access token
    #[must_use]
    pub fn access_token(&self) -> &String {
        self.inner_client.access_token()
    }

    /// Manually update the access token
    pub fn set_access_token(&mut self, access_token: impl Into<String>) {
        self.inner_client.set_access_token(access_token.into());
    }

    /// Return a list of accounts
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let client = Client::new(ACCESS_TOKEN);
    /// #
    /// let accounts = client.accounts().await?;
    /// #
    /// # Ok(())
    /// # }
    pub async fn accounts(&self) -> Result<Vec<accounts::Account>> {
        RequestBuilder::new(&self.inner_client, accounts::List)
            .send()
            .await
    }

    /// Return the balance of a given account
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let ACCOUNT_ID = "ACCOUNT_ID";
    /// # let client = Client::new(ACCESS_TOKEN);
    /// #
    /// let balance = client.balance(ACCOUNT_ID).await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn balance(&self, account_id: &str) -> Result<balance::Balance> {
        RequestBuilder::new(&self.inner_client, balance::Get::new(account_id))
            .send()
            .await
    }

    /// Return a list of Pots
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let ACCOUNT_ID = "ACCOUNT_ID";
    /// #
    /// # let client = Client::new(ACCESS_TOKEN);
    /// #
    /// let pots = client.pots(ACCOUNT_ID).await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pots(&self, account_id: &str) -> Result<Vec<pots::Pot>> {
        RequestBuilder::new(&self.inner_client, pots::List::new(account_id))
            .send()
            .await
    }

    /// Post a basic item on the account feed.
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::new(access_token);
    /// #
    /// let account_id = "ACCOUNT_ID";
    /// let title = "Feed Item Title";
    /// let image_url = "http://www.nyan.cat/cats/original.gif";
    ///
    /// client
    ///     .basic_feed_item(account_id, title, image_url)
    ///     .body("i figured out how to send messages to monzo from my computer...")
    ///     .send()
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn basic_feed_item<'a>(
        &'a self,
        account_id: &'a str,
        title: &'a str,
        image_url: &'a str,
    ) -> RequestBuilder<'a, M, feed_items::Basic<'a>> {
        RequestBuilder::new(
            &self.inner_client,
            feed_items::Basic::new(account_id, title, image_url),
        )
    }

    /// Deposit money into a pot
    pub async fn deposit_into_pot(
        &self,
        pot_id: &str,
        source_account_id: &str,
        amount: u32,
    ) -> Result<pots::Pot> {
        RequestBuilder::new(
            &self.inner_client,
            pots::Deposit::new(pot_id, source_account_id, amount),
        )
        .send()
        .await
    }

    /// Withdraw money from a pot
    pub async fn withdraw_from_pot(
        &self,
        pot_id: &str,
        destination_account_id: &str,
        amount: u32,
    ) -> Result<pots::Pot> {
        RequestBuilder::new(
            &self.inner_client,
            pots::Withdraw::new(pot_id, destination_account_id, amount),
        )
        .send()
        .await
    }

    /// Get a list of transactions
    ///
    /// The only required field is the account id, however optional pagination
    /// parameters can be supplied.
    ///
    /// # Example
    /// ```no_run
    /// use chrono::{Duration, Utc};
    /// use monzo::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::new(access_token);
    /// #
    /// let account_id = "ACCOUNT_ID";
    ///
    /// let transactions = client
    ///     .transactions(account_id)
    ///     .since(Utc::now() - Duration::days(10))
    ///     .limit(10)
    ///     .send()
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// *The Monzo API will only return transactions from more than 90 days ago
    /// in the first 5 minutes after authorising the Client. You can avoid this
    /// by using the 'since' method.*
    pub fn transactions<'a>(
        &'a self,
        account_id: &'a str,
    ) -> RequestBuilder<'a, M, transactions::List<'a>> {
        RequestBuilder::new(&self.inner_client, transactions::List::new(account_id))
    }

    /// Retrieve a transaction by transaction id
    ///
    /// # Example
    /// ```no_run
    /// use monzo::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::new(access_token);
    /// #
    /// let transaction_id = "TRANSACTION_ID";
    ///
    /// let transactions = client.transaction(transaction_id).send().await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// *The Monzo API will only return transactions from more than 90 days ago
    /// in the first 5 minutes after authorising the Client.
    pub fn transaction<'a>(
        &'a self,
        transaction_id: &'a str,
    ) -> RequestBuilder<'a, M, transactions::Get> {
        RequestBuilder::new(&self.inner_client, transactions::Get::new(transaction_id))
    }

    /// Return information about the current session
    pub async fn who_am_i(&self) -> Result<who_am_i::Response> {
        RequestBuilder::new(&self.inner_client, who_am_i::Request)
            .send()
            .await
    }
}
