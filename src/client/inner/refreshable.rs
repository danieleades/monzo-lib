use crate::{
    client,
    client::Client,
    endpoints::{auth, Endpoint},
    request_builder::RequestBuilder,
    Result,
};
use async_trait::async_trait;

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
    pub fn client_id(&self) -> &String {
        &self.inner_client.client_id
    }

    /// Get a reference to the client secret
    #[must_use]
    pub fn client_secret(&self) -> &String {
        &self.inner_client.client_secret
    }

    /// Get a reference to the refresh token
    #[must_use]
    pub fn refresh_token(&self) -> &String {
        &self.inner_client.refresh_token
    }

    /// Hit the Monzo auth endpoint and request new access and refresh tokens
    async fn get_refresh_tokens(&self) -> Result<auth::RefreshResponse> {
        RequestBuilder::new(
            &self.inner_client,
            auth::Refresh::new(self.client_id(), self.client_secret(), self.refresh_token()),
        )
        .send_no_auth()
        .await
    }

    /// Refresh the access and refresh tokens for this client
    pub async fn refresh_auth(&mut self) -> Result<()> {
        let response = self.get_refresh_tokens().await?;

        self.set_access_token(response.access_token);
        self.inner_client.refresh_token = response.refresh_token;

        Ok(())
    }
}

#[async_trait]
impl client::Inner for Refreshable {
    async fn execute(
        &self,
        endpoint: &dyn Endpoint,
        access_token: Option<&str>,
    ) -> reqwest::Result<reqwest::Response> {
        self.quick_client.execute(endpoint, access_token).await
    }

    fn access_token(&self) -> &String {
        self.quick_client.access_token()
    }

    fn set_access_token(&mut self, access_token: String) {
        self.quick_client.set_access_token(access_token);
    }
}
