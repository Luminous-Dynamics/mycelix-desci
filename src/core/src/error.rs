//! Error types for Mycelix-DeSci Core

use thiserror::Error;

/// Result type alias for Mycelix-DeSci operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for Mycelix-DeSci
#[derive(Error, Debug)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid epistemic tier
    #[error("Invalid epistemic tier: {0}")]
    InvalidEpistemicTier(String),

    /// Verification failed
    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    /// Cryptographic error
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// PoGQ error
    #[error("PoGQ error: {0}")]
    PoGQ(String),

    /// Trust calculation error
    #[error("Trust calculation error: {0}")]
    Trust(String),

    /// Invalid claim
    #[error("Invalid claim: {0}")]
    InvalidClaim(String),

    /// Not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Generic(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Generic(s)
    }
}
