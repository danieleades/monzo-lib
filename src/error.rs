use reqwest::StatusCode;

/// Common error type for anything that can go wrong with this crate
#[derive(Debug)]
pub enum Error {
    /// Errors associated with the underlying Reqwest crate
    Reqwest(reqwest::Error),

    /// Error returned when the access token has expired
    AuthExpired,

    /// HTTP client errors
    Client(StatusCode),

    /// HTTP server errors
    Server(StatusCode),
}

impl From<reqwest::Error> for Error {
    #[must_use]
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl From<reqwest::StatusCode> for Error {
    #[must_use]
    fn from(s: reqwest::StatusCode) -> Self {
        if s.is_client_error() {
            Self::Client(s)
        } else if s.is_server_error() {
            Self::Server(s)
        } else {
            panic!("this status code is not a valid error! ({})", s)
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => write!(f, "Reqwest Error: {}", e),
            Self::AuthExpired => write!(f, "Access token has expired!"),
            Self::Client(e) => write!(f, "Client error: {}", e),
            Self::Server(e) => write!(f, "Server error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    #[must_use]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reqwest(e) => e.source(),
            _ => None,
        }
    }
}
