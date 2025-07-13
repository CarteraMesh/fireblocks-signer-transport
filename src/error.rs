use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get vault account {0}")]
    FireblocksVaultError(String),

    #[error("Failed to deserialize solana message {0}")]
    InvalidMessage(String),

    #[error("No signature available {0}")]
    FireblocksNoSig(String),

    #[error("No pubkey for vault {0}")]
    FireblocksNoAddress(String),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    JwtError(#[from] crate::jwt::JwtError),

    #[error(transparent)]
    TokenError(#[from] jsonwebtoken::errors::Error),

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
