// User-owned controller for handler 'get_invoice_workflow_history'.

// Native untyped proxy route — delegates to shared k8s Service-targeted proxy (FR-20).
use brrtrouter::dispatcher::{HandlerRequest, HandlerResponse};
use brrtrouter::http::proxy_untyped;

pub fn handle(req: HandlerRequest) -> HandlerResponse {
    proxy_untyped(
        &req,
        "invoice",
        "/api/invoice/invoices/{id}/workflow-history",
    )
}
