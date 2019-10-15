#![warn(missing_docs)]
#![deny(clippy::all)]

//! [![Latest Docs](https://docs.rs/monzo-lib/badge.svg)](https://docs.rs/monzo-lib/)
//!
//! This crate is an async Monzo API client in pure rust.
//!
//! It is intended as the backend of a monzo CLI app that i'll probably
//! never get to building.
//!
//! In order to use this client, you will first need to get an access token and/or refresh token for the Monzo API (see [the docs](https://docs.monzo.com/))
//!
//! ## Usage
//! ```no_run
//! use monzo_lib::{Client, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = Client::builder()
//!         .access_token("ACCESS_TOKEN")
//!         .refresh_token("REFRESH_TOKEN")
//!         .build();
//!
//!     let accounts = client.accounts().await?;
//!
//!     let account_id = &accounts[0].id;
//!
//!     let balance = client.balance(account_id).await?;
//!
//!     Ok(())
//! }
//! ```

mod client;
pub use self::client::{accounts, balance, pots};
mod error;

pub use self::error::Error;

/// Result type for all methods in this crate which can fail.
pub type Result<T> = std::result::Result<T, Error>;

pub use self::client::{Client, ClientBuilder};
