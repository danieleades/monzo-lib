use serde::Serialize;

use crate::endpoints::Endpoint;

/// An object representing a request to the Monzo API for a list of accounts
pub struct Request<'a> {
    query: Query<'a>,
}

impl<'a> Request<'a> {
    pub(crate) const fn new(current_account_id: &'a str) -> Self {
        let query = Query { current_account_id };
        Self { query }
    }
}

impl Endpoint for Request<'_> {
    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn endpoint(&self) -> &'static str {
        "/pots"
    }

    fn query(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.query)
    }
}

#[derive(Debug, Serialize)]
struct Query<'a> {
    current_account_id: &'a str,
}
