#![deny(
    clippy::all,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    clippy::cargo
)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

//! A Monzo client in pure rust.
//!
//! It's ergonomic, strongly-typed, and asynchronous.
//!
//!
//! In order to use this client, you will first need to get an access token and/or refresh token for the Monzo API (see [the docs](https://docs.monzo.com/))
//!
//! ## Usage
//! ```no_run
//! use monzo::Client;
//!
//! #[tokio::main]
//! async fn main() -> monzo::Result<()> {
//!     // You can create a simple monzo client using only an access token
//!     let quick_client = Client::new("ACCESS_TOKEN");
//!
//!     // get a list of accounts
//!     let accounts = quick_client.accounts().await?;
//!
//!     // get the id of one of the accounts
//!     let account_id = &accounts[0].id;
//!
//!     // get the balance of that account
//!     let balance = quick_client.balance(account_id).await?;
//!
//!     // If you have a refresh token and client credentials
//!     // you can create or upgrade a client which is capable
//!     // of refreshing its own access token.
//!     let mut refreshable_client =
//!         quick_client.with_refresh_tokens("CLIENT_ID", "CLIENT_SECRET", "REFRESH_TOKEN");
//!
//!     refreshable_client.refresh_auth().await?;
//!
//!     Ok(())
//! }
//! ```

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod client;
#[doc(inline)]
pub use client::Client;
mod endpoints;
pub use endpoints::{
    accounts::{Account, Owner, Type as AccountType},
    balance::Balance,
    feed_items,
    pots::Pot,
    transactions,
    transactions::Transaction,
    who_am_i::Response as WhoAmI,
};
mod error;
pub use client::inner as inner_client;
pub use error::Error;

/// Result type for all methods in this crate which can fail.
pub type Result<T> = std::result::Result<T, Error>;
