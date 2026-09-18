// User-owned controller for handler 'execute_custom_report'.

use crate::handlers::execute_custom_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ExecuteCustomReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        completed_at: Some("example".to_string()),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        data: Some(Default::default()),
        error_message: Some("example".to_string()),
        id: "example".to_string(),
        period_end: Some("example".to_string()),
        period_start: Some("example".to_string()),
        report_id: "example".to_string(),
        status: "example".to_string(),
    })
}
