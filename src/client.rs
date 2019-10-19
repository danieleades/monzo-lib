use crate::{
    accounts::{AccountsRequestBuilder},
    auth::{RefreshRequest, RefreshResponse},
    balance::{Balance, BalanceRequest},
    endpoints::auth::RefreshTokens,
    pots::{Pots, PotsRequest},
    RequestBuilder, Result,
};
use reqwest::Client as HttpClient;

/// Client is a Monzo API client which requires only an access token.
///
/// The access token is valid for several hours after issue before it must be
/// refreshed.
///
/// see the [Client](monzo-lib::Client) for a client which is capable of
/// refreshing its own access.
pub struct Client {
    access_token: String,
    http_client: HttpClient,
    refresh_tokens: RefreshTokens,
}


impl Client {
    /// Create a new Monzo Client
    pub fn new(
        access_token: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        let http_client = HttpClient::new();
        let refresh_tokens = RefreshTokens::new(client_id, client_secret, refresh_token);
        Client {
            access_token: access_token.into(),
            http_client,
            refresh_tokens,
        }
    }

    /// Return a list of accounts
    pub fn accounts(&self) -> AccountsRequestBuilder {
        let endpoint = "https://api.monzo.com/accounts";

        self.http_client
            .get(endpoint)
            .bearer_auth(&self.access_token)
            .into()
    }

    /// Return the balance of a given account
    ///
    /// # Example
    /// ```no_run
    /// # use monzo_lib::Client;
    /// #
    /// # #[tokio::main]
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
    pub fn balance(&self, account_id: impl AsRef<str>) -> RequestBuilder<BalanceRequest, Balance> {
        let endpoint = "https://api.monzo.com/balance";

        self.http_client
            .get(endpoint)
            .bearer_auth(&self.access_token)
            .query(&[("account_id", account_id.as_ref())])
            .into()
    }

    /// Return a list of Pots
    ///
    /// # Example
    /// ```no_run
    /// # use monzo_lib::Client;
    /// #
    /// # #[tokio::main]
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
    pub fn pots(&self) -> RequestBuilder<PotsRequest, Pots> {
        let endpoint = "https://api.monzo.com/pots";

        self.http_client
            .get(endpoint)
            .bearer_auth(&self.access_token)
            .into()
    }

    fn get_refresh_tokens(&self) -> RequestBuilder<RefreshRequest, RefreshResponse> {
        self.http_client
            .post("https://api.monzo.com/oauth2/token")
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", &self.refresh_tokens.client_id),
                ("client_secret", &self.refresh_tokens.client_secret),
                ("refresh_token", &self.refresh_tokens.refresh_token),
            ])
            .into()
    }

    /// Refresh the access and refresh tokens for this client
    pub async fn refresh_auth(&mut self) -> Result<()> {
        let response = self.get_refresh_tokens().await?;

        self.access_token = response.access_token;
        self.refresh_tokens.refresh_token = response.refresh_token;

        Ok(())
    }
}
