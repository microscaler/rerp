// User-owned controller for handler 'create_report_definition'.

use crate::handlers::create_report_definition::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateReportDefinitionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        active: true,
        company_id: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
        report_type: "example".to_string(),
    }
}
