#![warn(missing_docs)]
#![warn(clippy::all)]

mod client;
mod error;

pub use self::error::Error;

/// Result type for all methods in this crate which can fail.
pub type Result<T> = std::result::Result<T, Error>;

pub use self::client::{Client, ClientBuilder};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
