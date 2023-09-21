use std::io;

use sea_orm::{DbErr, TransactionError};

#[derive(Debug, strum::Display)]
pub enum NotFoundCode {
    Any,
    Config,
    Movie,
    Tv,
    Season,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid argument, {0}")]
    InvalidArgument(String),

    #[error("not found, code: {0}, message: {1}")]
    NotFound(NotFoundCode, String),

    #[error("internal error, {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Internal(format!("reqwest error: {e}"))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Internal(format!("io error: {e}"))
    }
}

impl From<DbErr> for Error {
    fn from(e: DbErr) -> Self {
        Self::Internal(format!("db error: {e}"))
    }
}

impl From<TransactionError<Error>> for Error {
    fn from(e: TransactionError<Error>) -> Self {
        Self::Internal(format!("db transaction error: {e}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Internal(format!("serde_json error: {e}"))
    }
}
