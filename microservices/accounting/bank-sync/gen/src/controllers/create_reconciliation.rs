// User-owned controller for handler 'create_reconciliation'.

use crate::handlers::create_reconciliation::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateReconciliationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        adjusted_balance: Some(1.5),
        bank_account_id: "example".to_string(),
        book_balance: Some(1.5),
        created_at: Some("example".to_string()),
        difference: Some(1.5),
        id: "example".to_string(),
        matched_transactions: Some(42),
        notes: Some("example".to_string()),
        reconciled_at: Some("example".to_string()),
        reconciled_by: Some("example".to_string()),
        reconciliation_date: "example".to_string(),
        statement_balance: Some(1.5),
        statement_id: "example".to_string(),
        status: Some("example".to_string()),
        total_matched: Some(1.5),
        total_unmatched: Some(1.5),
        unmatched_transactions: Some(42),
        updated_at: Some("example".to_string()),
    })
}
