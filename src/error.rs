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
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl From<reqwest::StatusCode> for Error {
    fn from(s: reqwest::StatusCode) -> Error {
        if s.is_client_error() {
            Error::Client(s)
        } else if s.is_server_error() {
            Error::Server(s)
        } else {
            panic!("this status code is not a valid error! ({})", s)
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "Reqwest Error: {}", e),
            Error::AuthExpired => write!(f, "Access token has expired!"),
            Error::Client(e) => write!(f, "Client error: {}", e),
            Error::Server(e) => write!(f, "Server error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => e.source(),
            _ => None,
        }
    }
}
