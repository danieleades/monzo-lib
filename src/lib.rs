#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![feature(into_future)]

//! This crate is a Monzo client in pure rust.
//!
//! It's ergonomic, strongly-typed, and asynchronous.
//!
//! The majority of the endpoints are already supported. If you need a piece of
//! functionality that is not yet implemented, please open an issue or even
//! better, a pull request.
//!
//! In order to use this client, you will first need to get an access token and/or refresh token for the Monzo API (see [the docs](https://docs.monzo.com/))
//!
//! ## Usage
//! ```no_run
//! use monzo::{Client, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!
//!     // You can create a simple monzo client using only an access token
//!     let quick_client = Client::quick("ACCESS_TOKEN");
//!
//!     // get a list of accounts
//!     let accounts = quick_client.accounts().send().await?;
//!
//!     // get the id of one of the accounts
//!     let account_id = accounts[0].id();
//!
//!     // get the balance of that account
//!     let balance = quick_client.balance(account_id).send().await?;
//!
//!     // If you have a refresh token and client credentials
//!     // you can create or upgrade a client which is capable
//!     // of refreshing its own access token.
//!     let mut refreshable_client = quick_client.with_refresh_tokens(
//!         "CLIENT_ID",
//!         "CLIENT_SECRET",
//!         "REFRESH_TOKEN",
//!     );
//!
//!     refreshable_client.refresh_auth().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub use client::Client;
mod endpoints;
pub use endpoints::{accounts, auth, balance, feed_items, pots, transactions};
mod error;
pub use self::error::Error;

/// Result type for all methods in this crate which can fail.
pub type Result<T> = std::result::Result<T, Error>;
