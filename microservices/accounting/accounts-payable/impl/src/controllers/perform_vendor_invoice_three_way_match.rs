// Scaffold-only: honest 501 until product behavior is delivered (NFR-DOG-013).
// Replace this handler body when the owning work package implements the op.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_accounts_payable_gen::handlers::perform_vendor_invoice_three_way_match::Request;
use serde_json::Value;

#[handler(PerformVendorInvoiceThreeWayMatchController)]
pub fn handle(_request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    crate::http_support::not_implemented("perform_vendor_invoice_three_way_match")
}
