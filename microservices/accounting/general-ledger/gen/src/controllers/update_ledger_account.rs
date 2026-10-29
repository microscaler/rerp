// User-owned controller for handler 'update_ledger_account'.

use crate::handlers::update_ledger_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateLedgerAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
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
