// User-owned controller for handler 'get_dashboard'.

use crate::handlers::get_dashboard::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DashboardWidget;

#[handler(GetDashboardController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        id: "example".to_string(),
        is_default: Some(true),
        name: "example".to_string(),
        updated_at: Some("example".to_string()),
        user_id: "example".to_string(),
        widgets: vec![],
    })
}
