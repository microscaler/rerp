// User-owned controller for handler 'approve_payment_batch'.

// Native untyped proxy route — delegates to shared k8s Service-targeted proxy (FR-20).
use brrtrouter::dispatcher::{HandlerRequest, HandlerResponse};
use brrtrouter::http::proxy_untyped;

pub fn handle(req: HandlerRequest) -> HandlerResponse {
    proxy_untyped(
        &req,
        "accounts-payable",
        "/api/accounts-payable/payment-batches/{id}/approve",
    )
}
