// User-owned controller for handler 'create_reconciliation'.

use crate::handlers::create_reconciliation::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateReconciliationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        adjusted_balance: Some("example".to_string()),
        bank_account_id: "example".to_string(),
        book_balance: Some("example".to_string()),
        created_at: Some("example".to_string()),
        difference: Some("example".to_string()),
        id: "example".to_string(),
        matched_transactions: Some(42),
        notes: Some("example".to_string()),
        reconciled_at: Some("example".to_string()),
        reconciled_by: Some("example".to_string()),
        reconciliation_date: "example".to_string(),
        statement_balance: Some("example".to_string()),
        statement_id: "example".to_string(),
        status: Some("example".to_string()),
        total_matched: Some("example".to_string()),
        total_unmatched: Some("example".to_string()),
        unmatched_transactions: Some(42),
        updated_at: Some("example".to_string()),
    })
}
