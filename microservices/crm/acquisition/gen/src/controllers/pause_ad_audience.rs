// User-owned controller for handler 'pause_ad_audience'.

use crate::handlers::pause_ad_audience::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdAudienceSource;

#[handler(PauseAdAudienceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_id: "example".to_string(),
        consent_basis: Some("example".to_string()),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        exclude_source: Some(Default::default()),
        external_audience_id: Some("example".to_string()),
        id: "example".to_string(),
        identifiers: Some(vec![]),
        last_error: Some("example".to_string()),
        lookalike: Some(Default::default()),
        mode: Some("example".to_string()),
        name: "example".to_string(),
        platform: "example".to_string(),
        purpose: Some("example".to_string()),
        source: Default::default(),
        stats: Some(Default::default()),
        status: "example".to_string(),
        suppress_on: Some(vec![]),
        sync_interval_minutes: Some(42),
        updated_at: Some("example".to_string()),
        warm_up: Some(Default::default()),
    })
}
