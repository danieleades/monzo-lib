use super::request::Request;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A collection of Monzo pots
#[derive(Deserialize, Debug)]
pub struct Pots {
    pots: Vec<Pot>,
}

/// Representation of a Monzo pot
#[derive(Deserialize, Debug)]
pub struct Pot {
    pub id: String,
    pub name: String,
    pub style: String,
    pub balance: i64,
    pub currency: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub deleted: bool,
}

// Since there are no fields to set on this request, we simply forward the
// underlying 'Request'
pub(crate) type PotsRequest = Request<Pots>;
pub(crate) type PotDepositRequest = Request<Pot>;
