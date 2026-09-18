// User-owned controller for handler 'list_fiscal_periods'.

use crate::handlers::list_fiscal_periods::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FiscalPeriod;

#[handler(ListFiscalPeriodsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: vec![],
        limit: 42,
    })
}
