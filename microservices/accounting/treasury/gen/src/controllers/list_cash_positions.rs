// User-owned controller for handler 'list_cash_positions'.

use crate::handlers::list_cash_positions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CashPosition;

#[handler(ListCashPositionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    })
}
