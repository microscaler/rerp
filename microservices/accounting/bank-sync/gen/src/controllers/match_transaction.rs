// User-owned controller for handler 'match_transaction'.

use crate::handlers::match_transaction::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(MatchTransactionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        amount: 3.14,
        bank_account_id: Some("example".to_string()),
        counterparty_account: Some("example".to_string()),
        counterparty_name: Some("example".to_string()),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        description: Some("example".to_string()),
        dispute_reason: Some("example".to_string()),
        id: "example".to_string(),
        matched_by: Some("example".to_string()),
        matched_date: Some("example".to_string()),
        matched_entry_id: Some("example".to_string()),
        matched_entry_type: Some("example".to_string()),
        notes: Some("example".to_string()),
        reference: Some("example".to_string()),
        statement_id: "example".to_string(),
        status: Some("example".to_string()),
        transaction_date: "example".to_string(),
        updated_at: Some("example".to_string()),
        value_date: Some("example".to_string()),
    }
}
