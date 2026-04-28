// User-owned controller for handler 'complete_reconciliation'.

use crate::handlers::complete_reconciliation::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CompleteReconciliationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        adjusted_balance: Some(3.14),
        bank_account_id: "example".to_string(),
        book_balance: Some(3.14),
        created_at: Some("example".to_string()),
        difference: Some(3.14),
        id: "example".to_string(),
        matched_transactions: Some(42),
        notes: Some("example".to_string()),
        reconciled_at: Some("example".to_string()),
        reconciled_by: Some("example".to_string()),
        reconciliation_date: "example".to_string(),
        statement_balance: Some(3.14),
        statement_id: "example".to_string(),
        status: Some("example".to_string()),
        total_matched: Some(3.14),
        total_unmatched: Some(3.14),
        unmatched_transactions: Some(42),
        updated_at: Some("example".to_string()),
    }
}
