// User-owned controller for handler 'create_cash_forecast'.

use crate::handlers::create_cash_forecast::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateCashForecastController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        company_id: "example".to_string(),
        end_date: Some("example".to_string()),
        horizon: Default::default(),
        id: "example".to_string(),
        start_date: Some("example".to_string()),
        status: "example".to_string(),
    }
}
