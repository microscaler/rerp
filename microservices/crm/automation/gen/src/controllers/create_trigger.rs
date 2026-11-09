// User-owned controller for handler 'create_trigger'.

use crate::handlers::create_trigger::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TriggerType;

#[handler(CreateTriggerController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        enabled: true,
        entity: "example".to_string(),
        field: Some("example".to_string()),
        id: "example".to_string(),
        last_fired: Some("example".to_string()),
        schedule: Some("example".to_string()),
        r#type: Default::default(),
        workflow_id: "example".to_string(),
    })
}
