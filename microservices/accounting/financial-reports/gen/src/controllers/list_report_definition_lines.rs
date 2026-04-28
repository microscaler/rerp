// User-owned controller for handler 'list_report_definition_lines'.

use crate::handlers::list_report_definition_lines::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ReportDefinitionLine;

#[handler(ListReportDefinitionLinesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
