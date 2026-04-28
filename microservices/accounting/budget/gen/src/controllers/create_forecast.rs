// User-owned controller for handler 'create_forecast'.

use crate::handlers::create_forecast::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateForecastController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        as_of_date: "example".to_string(),
        budget_id: "example".to_string(),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        description: Some("example".to_string()),
        forecast_type: "example".to_string(),
        id: "example".to_string(),
        name: Some("example".to_string()),
        period_end: Some("example".to_string()),
        period_start: Some("example".to_string()),
        status: Some("example".to_string()),
        total_forecasted: Some(3.14),
        updated_at: Some("example".to_string()),
    }
}
