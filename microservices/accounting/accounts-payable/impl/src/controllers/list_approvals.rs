// Scaffold-only: honest 501 until product behavior is delivered (NFR-DOG-013).
// Replace this handler body when the owning work package implements the op.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_payable_gen::handlers::list_approvals::Request;
use serde_json::Value;

#[handler(ListApprovalsController)]
pub fn handle(_request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    crate::http_support::not_implemented("list_approvals")
}
