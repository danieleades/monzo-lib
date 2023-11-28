use erased_serde::Serialize as ErasedSerialize;

pub mod accounts;
pub mod auth;
pub mod balance;
pub mod feed_items;
pub mod pots;
pub mod transactions;
mod utils;
pub mod who_am_i;

pub trait Endpoint: Sync {
    const METHOD: reqwest::Method;
    const AUTH_REQUIRED: bool = true;
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
