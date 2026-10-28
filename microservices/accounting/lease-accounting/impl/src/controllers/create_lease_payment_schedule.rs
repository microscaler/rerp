// Scaffold-only: honest 501 until product behavior is delivered (NFR-DOG-013).
// Replace this handler body when the owning work package implements the op.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_lease_accounting_gen::handlers::create_lease_payment_schedule::Request;
use serde_json::Value;

#[handler(CreateLeasePaymentScheduleController)]
pub fn handle(_request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    crate::http_support::not_implemented("create_lease_payment_schedule")
}
