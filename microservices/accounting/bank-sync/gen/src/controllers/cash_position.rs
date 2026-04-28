// User-owned controller for handler 'cash_position'.

use crate::handlers::cash_position::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CashPositionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        as_of_date: "example".to_string(),
        available_balance: Some(3.14),
        by_account: Some(vec![]),
        company_id: "example".to_string(),
        currency_code: Some("example".to_string()),
        pending_transactions: Some(3.14),
        total_cash: 3.14,
    }
}
