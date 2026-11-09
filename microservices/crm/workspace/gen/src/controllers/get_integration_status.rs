// User-owned controller for handler 'get_integration_status'.

use crate::handlers::get_integration_status::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::IntegrationType;
#[allow(unused_imports)]
use crate::handlers::types::SyncDirection;
#[allow(unused_imports)]
use crate::handlers::types::SyncStatus;

#[handler(GetIntegrationStatusController)]
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
