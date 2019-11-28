//! Monzo API clients
//!
//! ## `[QuickClient](struct.QuickClient.html)`
//! For using with only an access token
//!
//! ## `[Client]`
//! for using with access token, refresh token, and client credentials

mod monzo_client;
pub use monzo_client::MonzoClient;
mod quick;
pub use quick::Client as Quick;
mod refreshable;
pub use refreshable::Client as Refreshable;
