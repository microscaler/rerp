// User-owned controller for handler 'get_contact_journey'.

use crate::handlers::get_contact_journey::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdConversion;
#[allow(unused_imports)]
use crate::handlers::types::AdTouchpoint;

#[handler(GetContactJourneyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "conversions": [],
    //   "devices": 0,
    //   "identifiers": {},
    //   "revenue": "0.00",
    //   "touchpoints": []
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "conversions": [],
  "devices": 0,
  "identifiers": {},
  "revenue": "0.00",
  "touchpoints": []
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        contact_id: Some("example".to_string()),
        contact_name: Some("example".to_string()),
        conversions: Some(vec![]),
        devices: Some(0),
        first_touch: Some(Default::default()),
        identifiers: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        last_touch: Some(Default::default()),
        model: Some("example".to_string()),
        revenue: Some("0.00".to_string()),
        touchpoints: Some(vec![]),
    })
}
