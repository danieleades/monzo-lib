use super::Pot;
use crate::endpoints::{Endpoint, Resolve};
use serde::{Deserialize, Serialize};

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
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> &str {
        "https://api.monzo.com/pots"
    }

    fn query(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.query)
    }
}

impl<'a> Resolve for Request<'a> {
    type Response = Vec<Pot>;

    fn resolve(&self, bytes: &[u8]) -> serde_json::Result<Self::Response> {
        #[derive(Deserialize)]
        struct Pots {
            pots: Vec<Pot>,
        }
        let pots: Pots = serde_json::from_slice(bytes)?;
        Ok(pots.pots)
    }
}

#[derive(Debug, Serialize)]
struct Query<'a> {
    current_account_id: &'a str,
}
