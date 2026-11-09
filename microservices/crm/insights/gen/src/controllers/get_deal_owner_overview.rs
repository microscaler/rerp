// User-owned controller for handler 'get_deal_owner_overview'.

use crate::handlers::get_deal_owner_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealOwnerRow;

#[handler(GetDealOwnerOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        currency: "example".to_string(),
        owners: vec![],
        period: Some("example".to_string()),
    })
}
