// User-owned controller for handler 'get_chart_of_account'.

// Native untyped proxy route — delegates to shared k8s Service-targeted proxy (FR-20).
use brrtrouter::dispatcher::{HandlerRequest, HandlerResponse};
use brrtrouter::http::proxy_untyped;

pub fn handle(req: HandlerRequest) -> HandlerResponse {
    proxy_untyped(
        &req,
        "general-ledger",
        "/api/general-ledger/chart-of-accounts/{id}",
    )
}
