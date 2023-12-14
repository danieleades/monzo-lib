//! Authentication endpoint

pub use refresh::{Request as Refresh, Response as RefreshResponse};

mod refresh {

    use serde::{Deserialize, Serialize};

    use crate::endpoints::Endpoint;

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
    pub struct Request<'a> {
        form: Form<'a>,
    }

    impl<'a> Request<'a> {
        pub(crate) const fn new(
            client_id: &'a str,
            client_secret: &'a str,
            refresh_token: &'a str,
        ) -> Self {
            let form = Form::new(client_id, client_secret, refresh_token);
            Self { form }
        }
    }

    impl<'a> Endpoint for Request<'a> {
        const AUTH_REQUIRED: bool = false;
        const METHOD: reqwest::Method = reqwest::Method::POST;

        fn endpoint(&self) -> &str {
            "/oauth2/token"
        }

        fn form(&self) -> Option<&dyn erased_serde::Serialize> {
            Some(&self.form)
        }
    }

    #[derive(Serialize)]
    struct Form<'a> {
        grant_type: &'static str,
        client_id: &'a str,
        client_secret: &'a str,
        refresh_token: &'a str,
    }

    impl<'a> Form<'a> {
        const fn new(client_id: &'a str, client_secret: &'a str, refresh_token: &'a str) -> Self {
            Self {
                grant_type: "refresh_token",
                client_id,
                client_secret,
                refresh_token,
            }
        }
    }
}
