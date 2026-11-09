// User-owned controller for handler 'record_ad_call'.

use crate::handlers::record_ad_call::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RecordAdCallController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        ad: Some("example".to_string()),
        ad_set: Some("example".to_string()),
        at: "example".to_string(),
        campaign: Some("example".to_string()),
        channel: Some("example".to_string()),
        credit: Some(1.5),
        detail: Some("example".to_string()),
        id: Some("example".to_string()),
        joined_by: Some("example".to_string()),
        kind: "example".to_string(),
        platform: Some("example".to_string()),
        url: Some("example".to_string()),
        value: Some(1.5),
    })
}
