// User-owned controller for handler 'generate_fiscal_year_periods'.

use crate::handlers::generate_fiscal_year_periods::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FiscalPeriod;

#[handler(GenerateFiscalYearPeriodsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        periods: Some(vec![]),
        periods_created: 42,
    })
}
