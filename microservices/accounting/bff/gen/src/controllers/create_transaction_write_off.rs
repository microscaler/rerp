// User-owned controller for handler 'create_transaction_write_off'.

// Native untyped proxy route — delegates to shared k8s Service-targeted proxy (FR-20).
use brrtrouter::dispatcher::{HandlerRequest, HandlerResponse};
use brrtrouter::http::proxy_untyped;

pub fn handle(req: HandlerRequest) -> HandlerResponse {
    proxy_untyped(
        &req,
        "bank-sync",
        "/api/bank-sync/transactions/{id}/write-off",
    )
}
