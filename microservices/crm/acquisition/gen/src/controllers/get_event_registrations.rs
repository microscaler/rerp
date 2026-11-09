// User-owned controller for handler 'get_event_registrations'.

use crate::handlers::get_event_registrations::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EventRegistration;

#[handler(GetEventRegistrationsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "attendee_email": "Attendee email",
    //       "event_name": "Event name",
    //       "id": "e67555dd-33f4-5fee-9da3-a019895219b1"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "attendee_email": "Attendee email",
      "event_name": "Event name",
      "id": "e67555dd-33f4-5fee-9da3-a019895219b1"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<EventRegistration>(serde_json::json!({"attendee_email":"Attendee email","event_name":"Event name","id":"e67555dd-33f4-5fee-9da3-a019895219b1"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
