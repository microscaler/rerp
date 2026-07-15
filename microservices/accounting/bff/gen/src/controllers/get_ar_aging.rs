// User-owned controller for handler 'get_ar_aging'.

// Native untyped proxy route — delegates to shared k8s Service-targeted proxy (FR-20).
use brrtrouter::dispatcher::{HandlerRequest, HandlerResponse};
use brrtrouter::http::proxy_untyped;

pub fn handle(req: HandlerRequest) -> HandlerResponse {
    proxy_untyped(
        &req,
        "accounts-receivable",
        "/api/accounts-receivable/ar-agings/{id}",
    )
}
