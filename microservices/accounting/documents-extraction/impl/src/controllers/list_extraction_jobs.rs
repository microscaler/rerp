// Scaffold-only: honest 501 until product behavior is delivered (NFR-DOG-013).
// Replace this handler body when the owning work package implements the op.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_documents_extraction_gen::handlers::list_extraction_jobs::Request;
use serde_json::Value;

#[handler(ListExtractionJobsController)]
pub fn handle(_request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    crate::http_support::not_implemented("list_extraction_jobs")
}
