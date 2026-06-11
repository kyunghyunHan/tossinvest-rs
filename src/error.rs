use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error [{code}]: {message}")]
    Api { code: String, message: String },

    #[error("Account not configured. Call with_account_seq() first.")]
    NoAccount,
}

pub type Result<T> = std::result::Result<T, Error>;
