// User-owned controller for handler 'create_currency_revaluation'.

use crate::handlers::create_currency_revaluation::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCurrencyRevaluationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: "example".to_string(),
        currency_code: "example".to_string(),
        id: "example".to_string(),
        journal_entry_id: Some("example".to_string()),
        status: "example".to_string(),
    })
}
