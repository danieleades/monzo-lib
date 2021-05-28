//! Monzo API clients
//!
//! ## [`QuickClient`]
//! For using with only an access token
//!
//! ## [`Client`]
//! for using with access token, refresh token, and client credentials

mod quick;
pub use quick::Client as QuickClient;
mod refreshable;
pub use refreshable::Client;
