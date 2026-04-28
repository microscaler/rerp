// User-owned controller for handler 'get_bank_statement'.

use crate::handlers::get_bank_statement::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetBankStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        bank_account_id: "example".to_string(),
        closing_balance: Some(3.14),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        file_reference: Some("example".to_string()),
        id: "example".to_string(),
        imported_at: Some("example".to_string()),
        imported_by: Some("example".to_string()),
        matched_count: Some(42),
        opening_balance: Some(3.14),
        start_date: Some("example".to_string()),
        statement_date: "example".to_string(),
        statement_number: Some("example".to_string()),
        status: Some("example".to_string()),
        transaction_count: Some(42),
        unmatched_count: Some(42),
        updated_at: Some("example".to_string()),
    }
}
