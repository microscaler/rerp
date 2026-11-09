// User-owned controller for handler 'resume_ad_buy'.

use crate::handlers::resume_ad_buy::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdBuySpec;

#[handler(ResumeAdBuyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        accounts: Some(vec![]),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
        purpose: Some("example".to_string()),
        spec: Default::default(),
        status: "example".to_string(),
        totals: Some(Default::default()),
        updated_at: Some("example".to_string()),
        warm_up_for_campaign_id: Some("example".to_string()),
    })
}
