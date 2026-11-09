// User-owned controller for handler 'record_ad_call'.

use crate::handlers::record_ad_call::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RecordAdCallController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "at": "2026-11-05T16:00:00Z",
    //   "kind": "AD_CLICK",
    //   "matched_contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //   "touchpoint_id": "6722b484-8b57-56f2-b588-3995ec2cbb3a"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "at": "2026-11-05T16:00:00Z",
  "kind": "AD_CLICK",
  "matched_contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
  "touchpoint_id": "6722b484-8b57-56f2-b588-3995ec2cbb3a"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        ad: Some("example".to_string()),
        ad_set: Some("example".to_string()),
        at: "2026-11-05T16:00:00Z".to_string(),
        campaign: Some("example".to_string()),
        channel: Some("example".to_string()),
        credit: Some(1.5),
        detail: Some("example".to_string()),
        id: Some("example".to_string()),
        joined_by: Some("example".to_string()),
        kind: "AD_CLICK".to_string(),
        matched_contact_id: Some("af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string()),
        platform: Some("example".to_string()),
        touchpoint_id: Some("6722b484-8b57-56f2-b588-3995ec2cbb3a".to_string()),
        url: Some("example".to_string()),
        value: Some("example".to_string()),
    })
}
