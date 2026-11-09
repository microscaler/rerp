// User-owned controller for handler 'create_tracking_domain'.

use crate::handlers::create_tracking_domain::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTrackingDomainController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "domain": "northwindtraders.example",
    //   "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //   "last_checked_at": "2026-11-05T16:00:00Z",
    //   "records": [
    //     {}
    //   ],
    //   "status": "PENDING",
    //   "tls": false
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "domain": "northwindtraders.example",
  "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
  "last_checked_at": "2026-11-05T16:00:00Z",
  "records": [
    {}
  ],
  "status": "PENDING",
  "tls": false
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        domain: "northwindtraders.example".to_string(),
        id: "3eb009b7-a99d-5a4c-956a-99e11d8dbb76".to_string(),
        last_checked_at: Some("2026-11-05T16:00:00Z".to_string()),
        records: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ]),
        status: "PENDING".to_string(),
        tls: Some(false),
    })
}
