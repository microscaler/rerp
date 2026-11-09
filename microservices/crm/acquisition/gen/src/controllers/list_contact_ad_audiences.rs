// User-owned controller for handler 'list_contact_ad_audiences'.

use crate::handlers::list_contact_ad_audiences::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdAudienceMember;

#[handler(ListContactAdAudiencesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [],
    //   "total": 0
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [],
  "total": 0
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![],
        total: Some(0),
    })
}
