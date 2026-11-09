// User-owned controller for handler 'build_report'.

use crate::handlers::build_report::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(BuildReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        data: Some(Default::default()),
        report_id: Some("example".to_string()),
        saved: Some(true),
    })
}
