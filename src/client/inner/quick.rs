use async_trait::async_trait;

use crate::{client, client::Client, endpoints::Endpoint};

/// A quick and dirty Monzo API client.
///
/// This client is easy to construct, because all you need is an access token.
/// This client is not capable of refreshing the access token, hence this must
/// be managed externally.
#[derive(Debug, Clone)]
#[must_use]
pub struct Quick {
    http_client: reqwest::Client,
    access_token: String,
    url: String,
}

impl Client<Quick> {
    /// Create a new Monzo Client..
    ///
    /// This `Client` needs only an access token to authenticate against
    /// the Monzo API, but is incapable of refreshing its access if the
    /// token expires.
    pub fn new(access_token: impl Into<String>) -> Self {
        let http_client = reqwest::Client::default();
        let inner_client = Quick {
            http_client,
            access_token: access_token.into(),
            url: "https://api.monzo.com".into(),
        };
        Self { inner_client }
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
    ) -> Client<client::inner::Refreshable> {
        Client::from_quick_client(self.inner_client, client_id, client_secret, refresh_token)
    }
}

#[async_trait]
impl client::Inner for Quick {
    async fn execute(&self, endpoint: &dyn Endpoint) -> reqwest::Result<reqwest::Response> {
        let mut request = self
            .http_client
            .request(endpoint.method(), self.url.clone() + endpoint.endpoint());

        if endpoint.auth_required() {
            request = request.bearer_auth(&self.access_token);
        }

        if let Some(query) = endpoint.query() {
            request = request.query(query);
        }

        if let Some(form) = endpoint.form() {
            request = request.form(form);
        }

        if let Some(json) = endpoint.json() {
            request = request.json(json);
        }

        request.send().await
    }

    fn access_token(&self) -> &String {
        &self.access_token
    }

    fn set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }

    fn url(&self) -> &str {
        &self.url
    }
}
