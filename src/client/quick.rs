use super::refreshable::Client as RefreshableClient;
use crate::{
    endpoints::{accounts, balance, feed_items, pots, transactions},
    Result,
};

/// A quick and dirty Monzo API client.
///
/// This client is easy to construct, because all you need is an access token.
/// This client is not capable of refreshing the access token, hence this must
/// be managed externally.
#[must_use]
pub struct Client {
    http_client: reqwest::Client,
    access_token: String,
}

impl Client {
    /// Create a new Monzo Client.
    ///
    /// This `Client` needs only an access token to authenticate against
    /// the Monzo API, but is incapable of refreshing its access if the
    /// token expires.
    pub fn new(access_token: impl Into<String>) -> Self {
        let http_client = reqwest::Client::new();
        Self::from_http_client(http_client, access_token)
    }

    /// BYO HTTP client.
    ///
    /// The Monzo client uses a reqwest http client under the hood. If you wish,
    /// you may use your own reqwest client with whatever configuration you see
    /// fit.
    pub fn from_http_client(http_client: reqwest::Client, access_token: impl Into<String>) -> Self {
        let access_token = access_token.into();

        Self {
            http_client,
            access_token,
        }
    }

    /// Upgrade a Client by adding refresh tokens.
    ///
    /// A client that has refresh tokens is able to refresh it's authentication
    /// when the access token expires.
    pub fn with_refresh_tokens(
        self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> RefreshableClient {
        RefreshableClient::from_quick_client(self, client_id, client_secret, refresh_token)
    }

    /// Return a reference to the current access token
    #[must_use]
    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    /// Return a list of accounts
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let client = Client::quick(ACCESS_TOKEN);
    /// #
    /// let accounts = client.accounts().await?;
    /// #
    /// # Ok(())
    /// # }
    pub async fn accounts(&self) -> Result<Vec<accounts::Account>> {
        accounts::List::new(self.http_client(), self.access_token())
            .send()
            .await
    }

    /// Return the balance of a given account
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let ACCOUNT_ID = "ACCOUNT_ID";
    /// # let client = Client::quick(ACCESS_TOKEN);
    /// #
    /// let balance = client.balance(ACCOUNT_ID).await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn balance(&self, account_id: &str) -> Result<balance::Balance> {
        balance::Get::new(self.http_client(), self.access_token(), account_id)
            .send()
            .await
    }

    /// Return a list of Pots
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let ACCOUNT_ID = "ACCOUNT_ID";
    /// #
    /// # let client = Client::quick(ACCESS_TOKEN);
    /// #
    /// let pots = client.pots(ACCOUNT_ID).await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pots(&self, account_id: &str) -> Result<Vec<pots::Pot>> {
        pots::List::new(self.http_client(), self.access_token(), account_id)
            .send()
            .await
    }

    /// Post a basic item on the account feed.
    ///
    /// # Example
    /// ```no_run
    /// use monzo::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::quick(access_token);
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
    ///
    /// # Note
    /// *At the time of writing the feed item API doesn't
    /// appear to quite match the documentation.
    /// 'image url' doesn't appear to do anything*
    #[must_use]
    pub fn basic_feed_item<'a>(
        &self,
        account_id: &'a str,
        title: &'a str,
        image_url: &'a str,
    ) -> feed_items::Basic<'a> {
        feed_items::Basic::new(
            self.http_client(),
            self.access_token(),
            account_id,
            title,
            image_url,
        )
    }

    /// Deposit money into a pot
    pub async fn deposit_into_pot(
        &self,
        pot_id: &str,
        source_account_id: &str,
        amount: i64,
    ) -> Result<pots::Pot> {
        pots::Deposit::new(
            self.http_client(),
            self.access_token(),
            pot_id,
            source_account_id,
            amount,
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
    /// # let client = Client::quick(access_token);
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
    #[must_use]
    pub fn transactions<'a>(&self, account_id: &'a str) -> transactions::List<'a> {
        transactions::List::new(self.http_client(), self.access_token(), account_id)
    }

    /// Retrieve a transaction by transaction id
    ///
    /// # Example
    /// ```no_run
    /// use monzo::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::quick(access_token);
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
    #[must_use]
    pub fn transaction(&self, transaction_id: &str) -> transactions::Get {
        transactions::Get::new(self.http_client(), self.access_token(), transaction_id)
    }

    /// Manually update the access token
    pub fn set_access_token(&mut self, access_token: impl Into<String>) {
        self.access_token = access_token.into();
    }

    /// Builder-style method for setting the access token
    pub fn with_access_token(mut self, access_token: impl Into<String>) -> Self {
        self.set_access_token(access_token);
        self
    }

    /// Return a reference to the internal http client
    #[must_use]
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// Swap out the internal http client for your own one.
    pub fn set_http_client(&mut self, http_client: reqwest::Client) {
        self.http_client = http_client;
    }
}
