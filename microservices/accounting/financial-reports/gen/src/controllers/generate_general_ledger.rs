// User-owned controller for handler 'generate_general_ledger'.

use crate::handlers::generate_general_ledger::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GenerateGeneralLedgerController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        entries: Some(vec![]),
        id: Some("example".to_string()),
        is_balanced: Some(true),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
        total_credits: Some(3.14),
        total_debits: Some(3.14),
    })
}
