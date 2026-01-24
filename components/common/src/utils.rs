// Common utility functions

use chrono::{DateTime, Utc};

pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn format_currency(amount: f64, currency: &str) -> String {
    format!("{currency} {amount:.2}")
}
