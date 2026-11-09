// User-owned controller for handler 'trigger_integration_sync'.

use crate::handlers::trigger_integration_sync::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TriggerIntegrationSyncController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        status: Some("example".to_string()),
        sync_id: Some("example".to_string()),
    })
}
