#![deny(
    clippy::all,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    clippy::cargo
)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
#![doc = include_str!("../README.md")]

mod client;
#[doc(inline)]
pub use client::Client;
mod endpoints;
#[doc(inline)]
pub use endpoints::accounts::{Account, Owner};
pub use endpoints::{
    accounts, balance::Balance, feed_items, pots::Pot, transactions, transactions::Transaction,
    who_am_i::Response as WhoAmI,
};
mod error;
pub use client::inner as inner_client;
pub use error::Error;

/// Result type for all methods in this crate which can fail.
pub type Result<T> = std::result::Result<T, Error>;
