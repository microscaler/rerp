// Scaffold-only: honest 501 until product behavior is delivered (NFR-DOG-013).
// Replace this handler body when the owning work package implements the op.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_bank_sync_gen::handlers::complete_reconciliation::Request;
use serde_json::Value;

#[handler(CompleteReconciliationController)]
pub fn handle(_request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    crate::http_support::not_implemented("complete_reconciliation")
}
