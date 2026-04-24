// User-owned controller for handler 'ledger_report'.

use crate::handlers::ledger_report::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::LedgerReportLine;

#[handler(LedgerReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        company_id: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        date_from: Some("example".to_string()),
        date_to: Some("example".to_string()),
        lines: Some(vec![]),
        period_id: Some("example".to_string()),
        report_date: Some("example".to_string()),
        total_lines: Some(42),
    }
}
