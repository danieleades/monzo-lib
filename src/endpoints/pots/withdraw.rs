use serde::Serialize;

use crate::endpoints::Endpoint;

pub struct Request<'a> {
    endpoint: String,
    form: Form<'a>,
}

impl<'a> Endpoint for Request<'a> {
    const METHOD: reqwest::Method = reqwest::Method::PUT;

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    fn form(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.form)
    }
}

impl<'a> Request<'a> {
    pub(crate) fn new(pot_id: &'a str, destination_account_id: &'a str, amount: u32) -> Self {
        use rand::{distributions::Alphanumeric, thread_rng, Rng};

        let endpoint = format!("/pots/{}/withdraw", &pot_id);

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
