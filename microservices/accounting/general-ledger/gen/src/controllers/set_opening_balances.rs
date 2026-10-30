// User-owned controller for handler 'set_opening_balances'.

use crate::handlers::set_opening_balances::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SetOpeningBalancesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        document_number: Some("example".to_string()),
        entry_number: "example".to_string(),
        fiscal_period_id: "example".to_string(),
        fiscal_year: 42,
        journal_entry_id: "example".to_string(),
        total_credit: "example".to_string(),
        total_debit: "example".to_string(),
    })
}
