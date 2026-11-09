// User-owned controller for handler 'create_integration'.

use crate::handlers::create_integration::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::IntegrationType;
#[allow(unused_imports)]
use crate::handlers::types::SyncDirection;
#[allow(unused_imports)]
use crate::handlers::types::SyncStatus;

#[handler(CreateIntegrationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        config: Some(Default::default()),
        created_at: Some("example".to_string()),
        id: "example".to_string(),
        is_active: Some(true),
        last_sync: Some("example".to_string()),
        name: "example".to_string(),
        sync_direction: Some(Default::default()),
        sync_status: Some(Default::default()),
        r#type: Default::default(),
    })
}
