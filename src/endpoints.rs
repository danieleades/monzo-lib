use erased_serde::Serialize as ErasedSerialize;

pub mod accounts;
pub mod auth;
pub mod balance;
pub(crate) mod feed_items;
pub mod pots;
pub mod transactions;
mod utils;
pub(crate) mod who_am_i;

pub trait Endpoint: Sync {
    fn method(&self) -> http::Method;
    fn endpoint(&self) -> &str;
    fn query(&self) -> Option<&dyn ErasedSerialize> {
        None
    }
    fn form(&self) -> Option<&dyn ErasedSerialize> {
        None
    }
    fn json(&self) -> Option<&dyn ErasedSerialize> {
        None
    }
}

pub trait Resolve {
    type Response;

    fn resolve(&self, bytes: &[u8]) -> serde_json::Result<Self::Response>;
}
