// User-owned controller for handler 'reopen_fiscal_year'.

use crate::handlers::reopen_fiscal_year::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ReopenFiscalYearController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        closed_at: Some("example".to_string()),
        closed_by: Some("example".to_string()),
        closing_date: Some("example".to_string()),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        description: Some("example".to_string()),
        end_date: "example".to_string(),
        id: "example".to_string(),
        is_closed: true,
        is_open: true,
        period_count: 42,
        start_date: "example".to_string(),
        updated_at: Some("example".to_string()),
        year: 42,
    })
}
