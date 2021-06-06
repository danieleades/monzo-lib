use super::Pot;
use crate::{endpoints::handle_response, Result};
/// An object representing a request to the Monzo API for a list of accounts
pub struct Request {
    request_builder: reqwest::RequestBuilder,
}

impl Request {
    pub(crate) fn new(
        http_client: &reqwest::Client,
        access_token: impl AsRef<str>,
        pot_id: &str,
        destination_account_id: &str,
        amount: i64,
    ) -> Self {
        use rand::{distributions::Alphanumeric, thread_rng, Rng};

        let dedupe_id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .map(char::from)
            .take(10)
            .collect();

        let request_builder = http_client
            .put(&format!("https://api.monzo.com/pots/{}/withdraw", pot_id))
            .bearer_auth(access_token.as_ref())
            .form(&[
                ("destination_account_id", destination_account_id),
                ("amount", &amount.to_string()),
                ("dedupe_id", &dedupe_id),
            ]);

        Self { request_builder }
    }

    /// Consume the request and a return a future that resolve to a [Pot] when
    /// the deposit has been completed
    pub async fn send(self) -> Result<Pot> {
        handle_response(self.request_builder).await
    }
}
