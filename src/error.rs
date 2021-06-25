use http::StatusCode;

/// Common error type for anything that can go wrong with this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP Errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Serialisation/Deserialisation Errors
    #[error("unable to serialise/deserialise struct")]
    Serde(#[from] serde_json::Error),

    /// Error returned when the access token has expired
    #[error("Access token has expired!")]
    AuthExpired,

    /// API client errors
    #[error("Client error: {0}")]
    Client(StatusCode),

    /// API server errors
    #[error("Server error: {0}")]
    Server(StatusCode),
}

impl From<StatusCode> for Error {
    fn from(status_code: StatusCode) -> Self {
        if status_code == StatusCode::UNAUTHORIZED {
            return Self::AuthExpired;
        } else if status_code.is_client_error() {
            return Self::Client(status_code);
        } else if status_code.is_server_error() {
            return Self::Server(status_code);
        }
        unimplemented!()
    }
}
