/// Common error type for anything that can go wrong with this crate
#[derive(Debug)]
pub enum Error {
    /// Errors associated with the underlying Reqwest crate
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "Reqwest Error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => e.source(),
        }
    }
}
