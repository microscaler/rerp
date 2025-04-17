// RERP Common Library
// Shared utilities and helpers for all RERP services

pub mod error;
pub mod validation;
pub mod utils;

// Re-export commonly used types
pub use error::{RerpError, RerpResult};
