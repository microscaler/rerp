// RERP Common Library
// Shared utilities and helpers for all RERP services

pub mod error;
pub mod utils;
pub mod validation;

// Re-export commonly used types
pub use error::{RerpError, RerpResult};
