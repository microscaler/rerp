// User-owned controller for handler 'register_event'.

use crate::handlers::register_event::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RegisterEventController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "attended": false,
    //   "attendee_company": "Attendee company",
    //   "attendee_email": "Attendee email",
    //   "attendee_function": "Attendee function",
    //   "attendee_name": "Attendee name",
    //   "attendee_phone": "Attendee phone",
    //   "event_date": "2026-11-05T16:00:00Z",
    //   "event_name": "Event name",
    //   "id": "9fa51332-6221-502c-a499-80ae6ddde550",
    //   "lead_id": "b68dbf73-2479-5234-b517-798dd1265706",
    //   "registration_date": "2026-11-05T16:00:00Z",
    //   "source": "Source"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "attended": false,
  "attendee_company": "Attendee company",
  "attendee_email": "Attendee email",
  "attendee_function": "Attendee function",
  "attendee_name": "Attendee name",
  "attendee_phone": "Attendee phone",
  "event_date": "2026-11-05T16:00:00Z",
  "event_name": "Event name",
  "id": "9fa51332-6221-502c-a499-80ae6ddde550",
  "lead_id": "b68dbf73-2479-5234-b517-798dd1265706",
  "registration_date": "2026-11-05T16:00:00Z",
  "source": "Source"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        attended: Some(false),
        attendee_company: Some("Attendee company".to_string()),
        attendee_email: "Attendee email".to_string(),
        attendee_function: Some("Attendee function".to_string()),
        attendee_name: Some("Attendee name".to_string()),
        attendee_phone: Some("Attendee phone".to_string()),
        event_date: Some("2026-11-05T16:00:00Z".to_string()),
        event_name: "Event name".to_string(),
        id: "9fa51332-6221-502c-a499-80ae6ddde550".to_string(),
        lead_id: Some("b68dbf73-2479-5234-b517-798dd1265706".to_string()),
        registration_date: Some("2026-11-05T16:00:00Z".to_string()),
        source: Some("Source".to_string()),
    })
}
