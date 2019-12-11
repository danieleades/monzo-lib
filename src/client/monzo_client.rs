use crate::{
    client::{quick::Client as QuickClient, refreshable::Client as RefreshableClient},
    endpoints::{accounts, balance, feed_items, pots, transactions},
};

/// This trait defines the shared behaviour of the client objects
pub trait MonzoClient {
    fn quick(access_token: impl Into<String>) -> QuickClient {
        QuickClient::new(access_token)
    }

    fn refreshable(
        access_token: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> RefreshableClient {
        RefreshableClient::new(access_token, client_id, client_secret, refresh_token)
    }

    /// Return a reference to the current access token
    #[must_use]
    fn access_token(&self) -> &String;

    /// Return a list of accounts
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, MonzoClient, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let client = Client::quick(ACCESS_TOKEN);
    /// #
    /// let accounts = client.accounts().send().await?;
    /// #
    /// # Ok(())
    /// # }
    #[must_use]
    fn accounts(&self) -> accounts::List;

    /// Return the balance of a given account
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, MonzoClient, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let ACCOUNT_ID = "ACCOUNT_ID";
    /// # let client = Client::quick(ACCESS_TOKEN);
    /// #
    /// let balance = client.balance(ACCOUNT_ID).send().await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    fn balance<'a>(&self, account_id: &'a str) -> balance::Get<'a>;

    /// Return a list of Pots
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, MonzoClient, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let client = Client::quick(ACCESS_TOKEN);
    /// #
    /// let pots = client.pots().send().await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    fn pots(&self) -> pots::List;

    /// Post a basic item on the account feed.
    ///
    /// # Example
    /// ```no_run
    /// use monzo::{Client, MonzoClient};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::quick(access_token);
    /// #
    /// let account_id = "ACCOUNT_ID";
    /// let title = "Feed Item Title";
    /// let image_url = "http://www.nyan.cat/cats/original.gif";
    ///
    /// client.basic_feed_item(
    ///     account_id,
    ///     title,
    ///     image_url,
    /// ).body("i figured out how to send messages to monzo from my computer...")
    /// .send().await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// *At the time of writing the feed item API doesn't
    /// appear to quite match the documentation.
    /// 'image url' doesn't appear to do anything*
    #[must_use]
    fn basic_feed_item<'a>(
        &self,
        account_id: &'a str,
        title: &'a str,
        image_url: &'a str,
    ) -> feed_items::Basic<'a>;

    /// Deposit money into a pot
    #[must_use]
    fn deposit_into_pot(&self, pot_id: &str, source_account_id: &str, amount: i64)
    -> pots::Deposit;

    /// Get a list of transactions
    ///
    /// The only required field is the account id, however optional pagination
    /// parameters can be supplied.
    ///
    /// # Example
    /// ```no_run
    /// use monzo::{Client, MonzoClient};
    /// use chrono::{Duration, Utc};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::quick(access_token);
    /// #
    /// let account_id = "ACCOUNT_ID";
    ///
    /// let transactions = client.transactions(account_id)
    ///     .since(Utc::now() - Duration::days(10))
    ///     .limit(10u16)
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
    #[must_use]
    fn transactions<'a>(&self, account_id: &'a str) -> transactions::List<'a>;

    /// Retrieve a transaction by transaction id
    ///
    /// # Example
    /// ```no_run
    /// use monzo::{MonzoClient, Client};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::quick(access_token);
    /// #
    /// let transaction_id = "TRANSACTION_ID";
    ///
    /// let transactions = client.transaction(transaction_id)
    ///     .send()
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// *The Monzo API will only return transactions from more than 90 days ago
    /// in the first 5 minutes after authorising the Client.
    #[must_use]
    fn transaction(&self, transaction_id: &str) -> transactions::Get;

    /// Manually update the access token
    fn set_access_token(&mut self, access_token: impl Into<String>);

    /// Builder-style method for setting the access token
    #[must_use]
    fn with_access_token(self, access_token: impl Into<String>) -> Self;

    /// Return a reference to the internal http client
    #[must_use]
    fn http_client(&self) -> &reqwest::Client;

    /// Swap out the internal http client for your own one.
    fn set_http_client(&mut self, http_client: reqwest::Client);
}
