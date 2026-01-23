// Common error types for RERP services

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RerpError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type RerpResult<T> = Result<T, RerpError>;
