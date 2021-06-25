use super::Pot;
use crate::endpoints::{Endpoint, Resolve};
use serde::Serialize;

pub struct Request<'a> {
    endpoint: String,
    form: Form<'a>,
}

impl<'a> Endpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::PUT
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    fn form(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.form)
    }
}

impl<'a> Resolve for Request<'a> {
    type Response = Pot;

    fn resolve(&self, bytes: &[u8]) -> serde_json::Result<Self::Response> {
        serde_json::from_slice(bytes)
    }
}

impl<'a> Request<'a> {
    pub(crate) fn new(pot_id: &'a str, destination_account_id: &'a str, amount: u32) -> Self {
        use rand::{distributions::Alphanumeric, thread_rng, Rng};

        let endpoint = format!("https://api.monzo.com/pots/{}/withdraw", &pot_id);

        let dedupe_id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .map(char::from)
            .take(10)
            .collect();

        let form = Form {
            destination_account_id,
            amount,
            dedupe_id,
        };

        Self { endpoint, form }
    }
}

#[derive(Debug, Serialize)]
struct Form<'a> {
    destination_account_id: &'a str,
    amount: u32,
    dedupe_id: String,
}
