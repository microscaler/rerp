// User-owned controller for handler 'variance_report'.

use crate::handlers::variance_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(VarianceReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        breach_threshold: Some(1.5),
        budget_id: "example".to_string(),
        budget_name: Some("example".to_string()),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        fiscal_year: Some(42),
        id: Some("example".to_string()),
        lines: Some(vec![]),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
        total_actual: Some(1.5),
        total_budgeted: Some(1.5),
        total_variance: Some(1.5),
        total_variance_percent: Some(1.5),
        warning_threshold: Some(1.5),
    })
}
