use super::QuickClient;
use crate::{
    endpoints::{accounts, auth, balance, feed_items, pots, transactions},
    Result,
};

/// A full-featured Monzo API client.
///
/// This client can refresh it's own access token if it expires
/// See the individual methods for descriptions of the API endpoints.
///
/// For a full list of client functionality, see the [MonzoClient] trait
#[must_use]
pub struct Client {
    quick_client: QuickClient,

    client_id: String,
    client_secret: String,
    refresh_token: String,
}

impl Client {
    /// Create a new `QuickClient`. The `QuickClient` can do everything that the
    /// normal client can do, except it cannot refresh its authentication of the
    /// access token expires (and it doesn't need the refresh token and client
    /// credentials to construct).
    ///
    /// This is functionally identical to calling `QuickClient::new(...)`
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, client::QuickClient};
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// #
    /// let client: QuickClient = Client::quick(ACCESS_TOKEN);
    /// ```
    pub fn quick(access_token: impl Into<String>) -> QuickClient {
        QuickClient::new(access_token)
    }

    /// Create a new `Client`.
    ///
    /// In order to create a refreshable client you will need an access token, a
    /// client ID, a client secret, and a refresh token. See the [Monzo API documentation](https://docs.monzo.com/) for details.
    ///
    /// It is possible to use a dummy string for the access token, provided the
    /// other details are correct and you call
    /// `[refresh_auth](Client::refresh_auth)` before using it. In practice,
    /// it's unlikely that you'll have refresh credentials and not also have
    /// an access token.
    pub fn new(
        access_token: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        QuickClient::new(access_token).with_refresh_tokens(client_id, client_secret, refresh_token)
    }

    /// BYO HTTP client.
    ///
    /// The Monzo client uses a reqwest http client under the hood. If you wish,
    /// you may use your own reqwest client with whatever configuration you see
    /// fit.
    pub fn from_http_client(
        http_client: reqwest::Client,
        access_token: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        QuickClient::from_http_client(http_client, access_token).with_refresh_tokens(
            client_id,
            client_secret,
            refresh_token,
        )
    }

    /// Get a reference to the client id in the request
    #[must_use]
    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    /// Get a reference to the client secret in the request
    #[must_use]
    pub fn client_secret(&self) -> &String {
        &self.client_secret
    }

    /// Get a reference to the refresh token in the request
    #[must_use]
    pub fn refresh_token(&self) -> &String {
        &self.refresh_token
    }

    /// Convenience method for creating a `Client` from a `QuickClient`
    pub(crate) fn from_quick_client(
        quick_client: QuickClient,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        Self {
            quick_client,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            refresh_token: refresh_token.into(),
        }
    }

    /// Hit the Monzo auth endpoint and request new access and refresh tokens
    #[must_use]
    fn get_refresh_tokens(&self) -> auth::Refresh {
        auth::Refresh::new(
            self.http_client(),
            self.client_id(),
            self.client_secret(),
            self.refresh_token(),
        )
    }

    /// Refresh the access and refresh tokens for this client
    pub async fn refresh_auth(&mut self) -> Result<()> {
        let response = self.get_refresh_tokens().send().await?;

        self.set_access_token(response.access_token);
        self.refresh_token = response.refresh_token;

        Ok(())
    }

    /// Return a reference to the current access token
    #[must_use]
    pub fn access_token(&self) -> &String {
        self.quick_client.access_token()
    }

    /// Return a list of accounts
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async pub fn main() -> Result<()> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let client = Client::quick(ACCESS_TOKEN);
    /// #
    /// let accounts = client.accounts().send().await?;
    /// #
    /// # Ok(())
    /// # }
    #[must_use]
    pub fn accounts(&self) -> accounts::List {
        self.quick_client.accounts()
    }

    /// Return the balance of a given account
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async pub fn main() -> Result<()> {
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
    pub fn balance<'a>(&self, account_id: &'a str) -> balance::Get<'a> {
        self.quick_client.balance(account_id)
    }

    /// Return a list of Pots
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async pub fn main() -> Result<()> {
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
    pub fn pots(&self) -> pots::List {
        self.quick_client.pots()
    }

    /// Post a basic item on the account feed.
    ///
    /// # Example
    /// ```no_run
    /// use monzo::Client;
    /// # #[tokio::main]
    /// # async pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn basic_feed_item<'a>(
        &self,
        account_id: &'a str,
        title: &'a str,
        image_url: &'a str,
    ) -> feed_items::Basic<'a> {
        self.quick_client
            .basic_feed_item(account_id, title, image_url)
    }

    /// Deposit money into a pot
    #[must_use]
    pub fn deposit_into_pot(
        &self,
        pot_id: &str,
        source_account_id: &str,
        amount: i64,
    ) -> pots::Deposit {
        self.quick_client
            .deposit_into_pot(pot_id, source_account_id, amount)
    }

    /// Get a list of transactions
    ///
    /// The only required field is the account id, however optional pagination
    /// parameters can be supplied.
    ///
    /// # Example
    /// ```no_run
    /// use monzo::Client;
    /// use chrono::{Duration, Utc};
    /// # #[tokio::main]
    /// # async pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::quick(access_token);
    /// #
    /// let account_id = "ACCOUNT_ID";
    ///
    /// let transactions = client.transactions(account_id)
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
        self.quick_client.transactions(account_id)
    }

    /// Retrieve a transaction by transaction id
    ///
    /// # Example
    /// ```no_run
    /// use monzo::Client;
    /// # #[tokio::main]
    /// # async pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn transaction(&self, transaction_id: &str) -> transactions::Get {
        transactions::Get::new(self.http_client(), self.access_token(), transaction_id)
    }

    /// Manually update the access token
    pub fn set_access_token(&mut self, access_token: impl Into<String>) {
        self.quick_client.set_access_token(access_token);
    }

    /// Builder-style method for setting the access token
    pub fn with_access_token(mut self, access_token: impl Into<String>) -> Self {
        self.set_access_token(access_token);
        self
    }

    /// Return a reference to the internal http client
    #[must_use]
    pub fn http_client(&self) -> &reqwest::Client {
        self.quick_client.http_client()
    }

    /// Swap out the internal http client for your own one.
    pub fn set_http_client(&mut self, http_client: reqwest::Client) {
        self.quick_client.set_http_client(http_client);
    }
}
