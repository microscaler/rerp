// User-owned controller for handler 'create_ledger_account'.

use crate::handlers::create_ledger_account::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateLedgerAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        account_type: "example".to_string(),
        active: true,
        code: "example".to_string(),
        control_role: Some("example".to_string()),
        currency_code: "example".to_string(),
        id: "example".to_string(),
        name: "example".to_string(),
        normal_side: "example".to_string(),
    })
}
