// User-owned controller for handler 'update_utms_campaign'.

use crate::handlers::update_utms_campaign::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateUtmsCampaignController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: Some(true),
        company_id: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
        sequence: Some(42),
    })
}
