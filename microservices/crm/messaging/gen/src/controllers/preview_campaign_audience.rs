// User-owned controller for handler 'preview_campaign_audience'.

use crate::handlers::preview_campaign_audience::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PreviewCampaignAudienceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        count: 42,
        excluded_by_segment: Some(42),
        excluded_suppressed: Some(42),
        excluded_unsubscribed: Some(42),
        sample: Some(vec![]),
    })
}
