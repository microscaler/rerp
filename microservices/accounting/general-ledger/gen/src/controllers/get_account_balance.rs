// User-owned controller for handler 'get_account_balance'.

use crate::handlers::get_account_balance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAccountBalanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_code: "example".to_string(),
        account_id: "example".to_string(),
        account_name: Some("example".to_string()),
        account_type: Some("example".to_string()),
        as_of_date: "example".to_string(),
        closing_balance: Some(3.14),
        currency_code: Some("example".to_string()),
        opening_balance: Some(3.14),
        period_credits: Some(3.14),
        period_debits: Some(3.14),
        period_id: Some("example".to_string()),
    })
}
