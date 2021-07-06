use crate::endpoints::Endpoint;
use serde::Serialize;

/// An object representing a request to the Monzo API for a list of accounts
pub struct Request<'a> {
    query: Query<'a>,
}

impl<'a> Request<'a> {
    pub(crate) fn new(current_account_id: &'a str) -> Self {
        let query = Query { current_account_id };
        Self { query }
    }
}

impl<'a> Endpoint for Request<'a> {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> &str {
        "https://api.monzo.com/pots"
    }

    fn query(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.query)
    }
}

#[derive(Debug, Serialize)]
struct Query<'a> {
    current_account_id: &'a str,
}
