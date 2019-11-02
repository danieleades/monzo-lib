//! Monzo API clients
//!
//! ## `[QuickClient](struct.QuickClient.html)`
//! For using with only an access token
//!
//! ## `[Client]`
//! for using with access token, refresh token, and client credentials

mod quick_client;
pub use quick_client::QuickClient;
mod refreshable_client;
pub use refreshable_client::Client;
