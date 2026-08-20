use serde::Serialize;

use crate::endpoints::Endpoint;

pub struct Request<'a> {
    endpoint: String,
    form: Form<'a>,
}

impl Endpoint for Request<'_> {
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
        use rand::{distr::Alphanumeric, rng, RngExt};

        let endpoint = format!("/pots/{pot_id}/withdraw");

        let dedupe_id: String = rng()
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

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::endpoints::Endpoint;

    #[test]
    fn builds_request() {
        let request = Request::new("pot_1234", "account_1234", 1_000);

        assert_eq!(request.endpoint(), "/pots/pot_1234/withdraw");
        assert_eq!(request.form.destination_account_id, "account_1234");
        assert_eq!(request.form.amount, 1_000);
        assert_eq!(request.form.dedupe_id.len(), 10);
        assert!(request
            .form
            .dedupe_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric()));
    }
}
