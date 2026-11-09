// User-owned controller for handler 'sync_ad_audience'.

use crate::handlers::sync_ad_audience::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SyncAdAudienceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        added: Some(42),
        audience_id: "example".to_string(),
        error: Some("example".to_string()),
        failed: Some(42),
        finished_at: Some("example".to_string()),
        id: "example".to_string(),
        platform_size: Some(42),
        removed: Some(42),
        started_at: Some("example".to_string()),
        status: "example".to_string(),
        trigger: Some("example".to_string()),
        uploaded_total: Some(42),
    })
}
