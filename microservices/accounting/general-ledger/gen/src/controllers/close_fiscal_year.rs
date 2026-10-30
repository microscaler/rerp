// User-owned controller for handler 'close_fiscal_year'.

use crate::handlers::close_fiscal_year::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CloseFiscalYearController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        document_number: "example".to_string(),
        entry_number: "example".to_string(),
        fiscal_year: 42,
        journal_entry_id: "example".to_string(),
        line_count: 42,
        net_income: "example".to_string(),
        retained_earnings_account_id: "example".to_string(),
    })
}
