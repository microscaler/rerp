// User-owned controller for handler 'generate_balance_sheet'.

use crate::handlers::generate_balance_sheet::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GenerateBalanceSheetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_details: Some(vec![]),
        assets: Some(Default::default()),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        equity: Some(Default::default()),
        id: Some("example".to_string()),
        liabilities: Some(Default::default()),
        period_end: "example".to_string(),
        period_start: Some("example".to_string()),
    })
}
