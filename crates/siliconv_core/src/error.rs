//! Error module for replay handling

use thiserror::Error;

#[derive(Debug, Error)]
/// Errors that can occur during replay reading or writing.
pub enum ReplayError {
    #[error("Failed to read replay: {0}")]
    /// Failed to read replay from source.
    ReadError(String),

    #[error("Failed to write replay: {0}")]
    /// Failed to write replay to destination.
    WriteError(String),

    /// Generic IO error.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
