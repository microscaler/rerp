// User-owned controller for handler 'create_report_export'.

use crate::handlers::create_report_export::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateReportExportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        artifact_uri: Some("example".to_string()),
        id: "example".to_string(),
        report_execution_id: "example".to_string(),
        status: "example".to_string(),
    }
}
