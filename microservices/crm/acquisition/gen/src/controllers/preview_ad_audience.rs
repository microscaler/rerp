// User-owned controller for handler 'preview_ad_audience'.

use crate::handlers::preview_ad_audience::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PreviewAdAudienceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        below_platform_minimum: Some(true),
        eligible: Some(42),
        estimated_match_rate: Some(1.5),
        estimated_matched: Some(42),
        excluded: Some(Default::default()),
        identifier_coverage: Some(Default::default()),
        source_count: Some(42),
    })
}
