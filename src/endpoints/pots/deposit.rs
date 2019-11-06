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
        source_account_id: &str,
        amount: i64,
    ) -> Self {
        use rand::{distributions::Alphanumeric, thread_rng, Rng};

        let dedupe_id: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        let request_builder = http_client
            .get(&format!("https://api.monzo.com/pots/{}/deposit", pot_id))
            .bearer_auth(access_token.as_ref())
            .form(&[
                ("source_account_id", source_account_id),
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
