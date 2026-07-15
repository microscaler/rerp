// User-owned controller for handler 'drill_down_report_cell'.

// Native untyped proxy route — delegates to shared k8s Service-targeted proxy (FR-20).
use brrtrouter::dispatcher::{HandlerRequest, HandlerResponse};
use brrtrouter::http::proxy_untyped;

pub fn handle(req: HandlerRequest) -> HandlerResponse {
    proxy_untyped(
        &req,
        "financial-reports",
        "/api/financial-reports/report-cells/{id}/drill-down",
    )
}
