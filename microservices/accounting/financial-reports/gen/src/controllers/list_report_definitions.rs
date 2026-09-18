// User-owned controller for handler 'list_report_definitions'.

use crate::handlers::list_report_definitions::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ReportDefinition;

#[handler(ListReportDefinitionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
