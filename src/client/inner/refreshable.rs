use async_trait::async_trait;

use crate::{
    client,
    client::{handle_request, Client},
    endpoints::{auth, Endpoint},
    Result,
};

/// A full-featured Monzo API client.
///
/// This client can refresh it's own access token if it expires
/// See the individual methods for descriptions of the API endpoints.
#[derive(Debug, Clone)]
#[must_use]
pub struct Refreshable {
    quick_client: client::inner::Quick,

    client_id: String,
    client_secret: String,
    refresh_token: String,
}

impl Client<Refreshable> {
    pub(crate) fn from_quick_client(
        quick_client: client::inner::Quick,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        let inner_client = Refreshable {
            quick_client,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            refresh_token: refresh_token.into(),
        };

        Self { inner_client }
    }

    /// Get a reference to the client id
    #[must_use]
    pub const fn client_id(&self) -> &String {
        &self.inner_client.client_id
    }

    /// Get a reference to the client secret
    #[must_use]
    pub const fn client_secret(&self) -> &String {
        &self.inner_client.client_secret
    }

    /// Get a reference to the refresh token
    #[must_use]
    pub const fn refresh_token(&self) -> &String {
        &self.inner_client.refresh_token
    }

    /// Hit the Monzo auth endpoint and request new access and refresh tokens
    async fn get_refresh_tokens(&self) -> Result<auth::RefreshResponse> {
        handle_request(
            &self.inner_client,
            &auth::Refresh::new(self.client_id(), self.client_secret(), self.refresh_token()),
        )
        .await
    }

    /// Refresh the access and refresh tokens for this client
    ///
    /// Returns the time (in seconds) until the token expires
    pub async fn refresh_auth(&mut self) -> Result<i64> {
        let response = self.get_refresh_tokens().await?;
        let expires_in = response.expires_in;

        self.set_access_token(response.access_token);
        self.inner_client.refresh_token = response.refresh_token;

        Ok(expires_in)
    }
}

#[async_trait]
impl client::Inner for Refreshable {
    async fn execute(&self, endpoint: &dyn Endpoint) -> reqwest::Result<reqwest::Response> {
        self.quick_client.execute(endpoint).await
    }

    fn access_token(&self) -> &String {
        self.quick_client.access_token()
    }

    fn set_access_token(&mut self, access_token: String) {
        self.quick_client.set_access_token(access_token);
    }

    fn url(&self) -> &str {
        self.quick_client.url()
    }
}
