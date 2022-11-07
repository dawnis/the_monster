use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("{0}")]
    StringError(String),
    #[error("expected {expected:?}, got {found:?}")]
    NumberMismatch { expected: u8, found: u8},
    #[error("Unknown error")]
    ErrorOther,
}
