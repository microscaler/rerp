// User-owned controller for handler 'reconcile_transaction'.

use crate::handlers::reconcile_transaction::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ReconcileTransactionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        journal_entry_id: Some("example".to_string()),
        reconciliation_id: Some("example".to_string()),
        status: "example".to_string(),
        transaction_id: "example".to_string(),
    })
}
