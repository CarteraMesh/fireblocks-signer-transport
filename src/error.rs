use thiserror::Error;

#[derive(Debug, Error)]
pub enum FireblocksClientError {
    #[error("Failed to get vault account {0}")]
    FireblocksVaultError(String),

    #[error("No signature available {0}")]
    FireblocksNoSig(String),

    #[error("No pubkey for vault {0}")]
    FireblocksNoAddress(String),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    JwtError(#[from] crate::jwt::JwtError),

    #[error(
        "Secret key format is invalid. Expected RSA private key in PEM format (-----BEGIN RSA PRIVATE KEY-----): {0}"
    )]
    TokenError(#[from] jsonwebtoken::errors::Error),

    #[error("API key must be a valid UUID v4 format: {0}")]
    InvalidApiKey(String),

    #[error("{0}")]
    FireblocksServerError(String),

    #[error("{0}")]
    JsonParseErr(String),

    #[error(transparent)]
    JsonErr(#[from] serde_json::Error),

    #[error("Operation timed out")]
    Timeout,

    #[error("pubkey on lookuptable is invalid")]
    InvalidPubkey,

    #[error(transparent)]
    EnvMissing(#[from] std::env::VarError),

    #[error("Unknown asset {0}")]
    UnknownAsset(String),
}
