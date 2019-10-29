use super::QuickClient;
use crate::{auth::RefreshAuth, Result};

/// A full-featured Monzo API client.
///
/// This client can refresh it's own access token if it expires
/// See the individual methods for descriptions of the API endpoints.
pub struct Client {
    quick_client: QuickClient,

    client_id: String,
    client_secret: String,
    refresh_token: String,
}

impl std::ops::Deref for Client {
    type Target = QuickClient;
    fn deref(&self) -> &Self::Target {
        &self.quick_client
    }
}
impl std::ops::DerefMut for Client {
    fn deref_mut(&mut self) -> &mut QuickClient {
        &mut self.quick_client
    }
}

impl Client {
    /// Create a new 'QuickClient'. The QuickClient can do everything that the
    /// normal client can do, except it cannot refresh its authentication of the
    /// access token expires (and it doesn't need the refresh token and client
    /// credentials to construct).
    ///
    /// This is functionally identical to calling `QuickClient::new(...)`
    ///
    /// # Example
    /// ```no_run
    /// # use monzo_lib::{Client, client::QuickClient};
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// #
    /// let client: QuickClient = Client::quick(ACCESS_TOKEN);
    /// ```
    pub fn quick(access_token: impl Into<String>) -> QuickClient {
        QuickClient::new(access_token)
    }

    /// Create a new Client.
    ///
    /// In order to create a refreshable client you will need an access token, a
    /// client ID, a client secret, and a refresh token. See the [Monzo API documentation](https://docs.monzo.com/) for details.
    ///
    /// It is possible to use a dummy string for the access token, provided the
    /// other details are correct and you call
    /// [refresh_auth](Client::refresh_auth) before using it. In practice, it's
    /// unlikely that you'll have refresh credentials and not also have an
    /// access token.
    pub fn new(
        access_token: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        QuickClient::new(access_token).with_refresh_tokens(client_id, client_secret, refresh_token)
    }

    /// Get a reference to the client_id in the request
    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    /// Get a reference to the client_secret in the request
    pub fn client_secret(&self) -> &String {
        &self.client_secret
    }

    /// Get a reference to the refresh token in the request
    pub fn refresh_token(&self) -> &String {
        &self.refresh_token
    }

    /// Convenience method for creating a Client from a QuickClient
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
    fn get_refresh_tokens(&self) -> RefreshAuth {
        RefreshAuth::new(
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
}
