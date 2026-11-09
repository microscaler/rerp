// User-owned controller for handler 'create_goal'.

use crate::handlers::create_goal::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateGoalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        conversion_id: "example".to_string(),
        conversion_name: Some("example".to_string()),
        created_at: Some("example".to_string()),
        current: Some(rust_decimal::Decimal::new(12345, 2)),
        drivers: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
        note: Some("example".to_string()),
        period: Some("example".to_string()),
        period_end: Some("example".to_string()),
        period_start: Some("example".to_string()),
        status: Some("example".to_string()),
        target: rust_decimal::Decimal::new(12345, 2),
        unit: Some("example".to_string()),
    })
}
