// User-owned controller for handler 'list_fiscal_year_periods'.

use crate::handlers::list_fiscal_year_periods::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FiscalPeriod;

#[handler(ListFiscalYearPeriodsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
