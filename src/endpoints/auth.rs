//! Authentication endpoint

use crate::{endpoints::handle_response, Result};
use reqwest::Client as HttpClient;
use serde::Deserialize;

/// The response received from the Monzo API after a successful request to
/// refresh the authentication.
#[derive(Deserialize, Debug)]
pub struct RefreshResponse {
    /// New access token for authorising requests against the Monzo API
    pub access_token: String,

    /// The id of the client
    pub client_id: String,

    /// time (in seconds) until the access token expires
    pub expires_in: i64,

    /// Refresh token. This token can be used to generate a new access/refresh
    /// token pair
    pub refresh_token: String,

    /// The token type. currently the only supported token type is "bearer_auth"
    pub token_type: String,

    /// The id of the current Monzo user
    pub user_id: String,
}

/// A request for new authentication tokens.
///
/// This struct implements [IntoFuture](std::future::IntoFuture), and can be
/// `await`ed directly.
pub struct RefreshAuth {
    reqwest_builder: reqwest::RequestBuilder,
}

impl RefreshAuth {
    pub(crate) fn new(
        http_client: &HttpClient,
        client_id: impl AsRef<str>,
        client_secret: impl AsRef<str>,
        refresh_token: impl AsRef<str>,
    ) -> Self {
        let reqwest_builder = http_client
            .post("https://api.monzo.com/oauth2/token")
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", client_id.as_ref()),
                ("client_secret", client_secret.as_ref()),
                ("refresh_token", refresh_token.as_ref()),
            ]);

        Self { reqwest_builder }
    }

    /// Send the response to the Monzo server.
    ///
    /// This method consumes the request and produces a future which will
    /// resolve to a [RefreshResponse]. This method is effectively an alias
    /// for the `into_future` method.
    pub async fn send(self) -> Result<RefreshResponse> {
        handle_response(self.reqwest_builder).await
    }
}
