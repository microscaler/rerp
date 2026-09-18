// User-owned controller for handler 'drill_down_report_cell'.

use crate::handlers::drill_down_report_cell::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DrillDownReportCellController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        cell_id: "example".to_string(),
        source_lines: vec![],
    }
}
