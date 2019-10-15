use reqwest::{Client as HttpClient, RequestBuilder};

pub mod accounts;
pub mod balance;
pub mod pots;
mod request;

use self::{
    accounts::{Accounts, AccountsRequest},
    balance::{Balance, BalanceRequest},
    pots::{Pots, PotsRequest},
};
use crate::Result;

fn get_endpoint(endpoint: impl AsRef<str> + 'static) -> String {
    format!("https://api.monzo.com/{}", endpoint.as_ref())
}

/// The ClientBuilder is used for configuring and constructing a new Monzo
/// Client
#[derive(Default)]
pub struct ClientBuilder {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl ClientBuilder {
    /// Set the access token for querying the Monzo API. This is required if the
    /// refresh token is not set
    pub fn access_token(mut self, access_token: impl Into<String>) -> Self {
        self.access_token = Some(access_token.into());
        self
    }

    /// Set the refresh token for generating a new access token. This is
    /// required if the access token is not provided
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = Some(refresh_token.into());
        self
    }

    /// consumes the ClientBuilder and returns a Client.
    ///
    /// # Panics
    /// This method will panic if neither the access token or refresh token are
    /// set
    ///
    /// # Note
    /// Currently this naively assumes that the access token is provided, and
    /// valid. no logic for refreshing the token is yet implemented.
    pub fn build(self) -> Client {
        // for now, assume we have both
        let access_token = self.access_token.unwrap();
        let refresh_token = self.refresh_token;

        let http_client = HttpClient::new();

        Client {
            http_client,
            access_token,
            refresh_token,
        }
    }
}

/// Monzo Client
pub struct Client {
    http_client: HttpClient,
    access_token: String,
    refresh_token: Option<String>,
}

impl Client {
    /// Returns a new Monzo Client.
    ///
    /// For more fine-grained control over the client configuration, see the
    /// builder API.
    pub fn new(access_token: impl Into<String>) -> Self {
        Client::builder().access_token(access_token).build()
    }

    /// return a ClientBuilder for configuring a new Client
    ///
    /// The builder API can be used for more fine-grained control over the
    /// configuration of the client.
    ///
    /// # Example
    /// ```no_run
    /// use monzo_lib::Client;
    /// #
    /// #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::builder()
    ///        .access_token("ACCESS_TOKEN")
    ///        .refresh_token("REFRESH_TOKEN")
    ///        .build();
    /// #   Ok(())
    /// # }
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    fn get(&self, endpoint: impl AsRef<str> + 'static) -> RequestBuilder {
        let endpoint = get_endpoint(endpoint);

        self.http_client
            .get(&endpoint)
            .bearer_auth(&self.access_token)
    }

    /// Return a list of bank accounts associated with the Monzo account
    ///
    /// # Example
    /// ```no_run
    /// # use monzo_lib::Client;
    /// #
    /// #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #    let client = Client::builder()
    /// #       .access_token("ACCESS_TOKEN")
    /// #       .refresh_token("REFRESH_TOKEN")
    /// #       .build();
    /// #
    ///    let accounts = client.accounts().await?;
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub async fn accounts(&self) -> Result<Accounts> {
        AccountsRequest::from(self.get("accounts")).await
    }

    /// Return the balance of a given account
    ///
    /// # Example
    /// ```no_run
    /// # use monzo_lib::Client;
    /// #
    /// #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #    let client = Client::builder()
    /// #       .access_token("ACCESS_TOKEN")
    /// #       .refresh_token("REFRESH_TOKEN")
    /// #       .build();
    /// #
    ///    let account_balance = client.balance("ACCOUNT_ID").await?;
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub async fn balance(&self, account_id: impl AsRef<str>) -> Result<Balance> {
        BalanceRequest::from(
            self.get("balance")
                .query(&[("account_id", account_id.as_ref())]),
        )
        .await
    }

    /// Return a list of Pots
    ///
    /// # Example
    /// ```no_run
    /// # use monzo_lib::Client;
    /// #
    /// #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #    let client = Client::builder()
    /// #       .access_token("ACCESS_TOKEN")
    /// #       .refresh_token("REFRESH_TOKEN")
    /// #       .build();
    /// #
    ///    let pots = client.pots().await?;
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub async fn pots(&self) -> Result<Pots> {
        PotsRequest::from(self.get("pots")).await
    }
}
