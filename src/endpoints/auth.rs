//! Authentication endpoint

pub use refresh::{Request as Refresh, Response as RefreshResponse};

mod refresh {

    use crate::{endpoints::handle_response, Result};
    use reqwest::Client as HttpClient;
    use serde::{Deserialize, Serialize};

    /// The response received from the Monzo API after a successful request to
    /// refresh the authentication.
    #[derive(Deserialize, Debug)]
    pub struct Response {
        /// New access token for authorising requests against the Monzo API
        pub access_token: String,

        /// The id of the client
        pub client_id: String,

        /// time (in seconds) until the access token expires
        pub expires_in: i64,

        /// Refresh token. This token can be used to generate a new
        /// access/refresh token pair
        pub refresh_token: String,

        /// The token type. currently the only supported token type is
        /// "bearer_auth"
        pub token_type: String,

        /// The id of the current Monzo user
        pub user_id: String,
    }

    /// A request for new authentication tokens.
    pub struct Request {
        reqwest_builder: reqwest::RequestBuilder,
    }

    #[derive(Serialize)]
    struct Payload<'a> {
        grant_type: &'static str,
        client_id: &'a str,
        client_secret: &'a str,
        refresh_token: &'a str,
    }

    impl<'a> Payload<'a> {
        fn new(client_id: &'a str, client_secret: &'a str, refresh_token: &'a str) -> Self {
            Payload {
                grant_type: "refresh_token",
                client_id,
                client_secret,
                refresh_token,
            }
        }
    }

    impl Request {
        pub(crate) fn new(
            http_client: &HttpClient,
            client_id: &str,
            client_secret: &str,
            refresh_token: &str,
        ) -> Self {
            let payload = Payload::new(client_id, client_secret, refresh_token);

            let reqwest_builder = http_client
                .post("https://api.monzo.com/oauth2/token")
                .form(&payload);

            Self { reqwest_builder }
        }

        /// Send the response to the Monzo server.
        ///
        /// This method consumes the request and produces a future which will
        /// resolve to a `[RefreshResponse]`. This method is effectively an
        /// alias for the `into_future` method.
        pub async fn send(self) -> Result<Response> {
            handle_response(self.reqwest_builder).await
        }
    }
}
