use super::{refreshable::Client as RefreshableClient, MonzoClient};
use crate::endpoints::{accounts, balance, feed_items, pots, transactions};
use reqwest::Client as HttpClient;

/// A quick and dirty Monzo API client.
///
/// This client is easy to construct, because all you need is an access token.
/// This client is not capable of refreshing the access token, hence this must
/// be managed externally.
///
/// For a full list of client functionality, see the [MonzoClient] trait
#[must_use]
pub struct Client {
    http_client: HttpClient,
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

    /// BYO HTTP client.#~~~
    ///
    /// The Monzo client uses a reqwest http client under the hood. If you wish,
    /// you may use your own reqwest client with whatever configuration you see
    /// fit.
    pub fn from_http_client(http_client: HttpClient, access_token: impl Into<String>) -> Self {
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
}

impl MonzoClient for Client {
    /// Return a reference to the current access token
    #[must_use]
    fn access_token(&self) -> &String {
        &self.access_token
    }

    #[must_use]
    fn accounts(&self) -> accounts::List {
        accounts::List::new(self.http_client(), self.access_token())
    }

    #[must_use]
    fn balance<'a>(&self, account_id: &'a str) -> balance::Get<'a> {
        balance::Get::new(self.http_client(), self.access_token(), account_id)
    }

    #[must_use]
    fn pots(&self) -> pots::List {
        pots::List::new(self.http_client(), self.access_token())
    }

    #[must_use]
    fn basic_feed_item<'a>(
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
    #[must_use]
    fn deposit_into_pot(
        &self,
        pot_id: &str,
        source_account_id: &str,
        amount: i64,
    ) -> pots::Deposit {
        pots::Deposit::new(
            self.http_client(),
            self.access_token(),
            pot_id,
            source_account_id,
            amount,
        )
    }

    #[must_use]
    fn transactions<'a>(&self, account_id: &'a str) -> transactions::List<'a> {
        transactions::List::new(self.http_client(), self.access_token(), account_id)
    }

    #[must_use]
    fn transaction(&self, transaction_id: &str) -> transactions::Get {
        transactions::Get::new(self.http_client(), self.access_token(), transaction_id)
    }

    /// Manually update the access token
    fn set_access_token(&mut self, access_token: impl Into<String>) {
        self.access_token = access_token.into();
    }

    /// Builder-style method for setting the access token
    fn with_access_token(mut self, access_token: impl Into<String>) -> Self {
        self.set_access_token(access_token);
        self
    }

    /// Return a reference to the internal http client
    #[must_use]
    fn http_client(&self) -> &HttpClient {
        &self.http_client
    }

    /// Swap out the internal http client for your own one.
    fn set_http_client(&mut self, http_client: HttpClient) {
        self.http_client = http_client;
    }
}
