// User-owned controller for handler 'create_pipeline'.

use crate::handlers::create_pipeline::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreatePipelineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        auto_assign: Some("example".to_string()),
        auto_assign_user_ids: Some(vec![]),
        created_at: "example".to_string(),
        currency: "example".to_string(),
        forecast_mode: Some("example".to_string()),
        group_ids: Some(vec![]),
        id: "example".to_string(),
        name: "example".to_string(),
        open_deals: Some(42),
        open_value: Some(1.5),
        sequence: Some(42),
        updated_at: Some("example".to_string()),
        visibility: Some("example".to_string()),
    })
}
