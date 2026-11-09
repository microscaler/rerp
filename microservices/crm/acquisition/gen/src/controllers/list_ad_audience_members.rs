// User-owned controller for handler 'list_ad_audience_members'.

use crate::handlers::list_ad_audience_members::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdAudienceMember;

#[handler(ListAdAudienceMembersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 0
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [],
  "limit": 20,
  "page": 1,
  "total": 0
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![],
        limit: 20,
        page: 1,
        total: 0,
    })
}
