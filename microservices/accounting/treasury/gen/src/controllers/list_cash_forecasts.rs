// User-owned controller for handler 'list_cash_forecasts'.

use crate::handlers::list_cash_forecasts::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListCashForecastsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
