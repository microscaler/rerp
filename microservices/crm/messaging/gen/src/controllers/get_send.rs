// User-owned controller for handler 'get_send'.

use crate::handlers::get_send::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::MessageEvent;
#[allow(unused_imports)]
use crate::handlers::types::Provider;
#[allow(unused_imports)]
use crate::handlers::types::SendStatus;

#[handler(GetSendController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "campaign_id": "94cb11b7-4223-5b33-bb8d-320012c437a0",
    //   "campaign_name": "Campaign name",
    //   "channel": "EMAIL",
    //   "clicked_at": "2026-11-05T16:00:00Z",
    //   "clicked_link": "Clicked link",
    //   "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
    //   "contact_name": "Contact name",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "enrolment_id": "a8314e09-6da8-5b0a-8ac0-8ffd966cf280",
    //   "error": "Error",
    //   "events": [
    //     {
    //       "at": "2026-11-05T16:00:00Z",
    //       "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
    //       "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //       "send_id": "a28a1db3-cee2-594e-adbd-25b5f5f34365",
    //       "type": "QUEUED"
    //     }
    //   ],
    //   "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //   "opened_at": "2026-11-05T16:00:00Z",
    //   "provider": "SES",
    //   "provider_message_id": "ext_e7e895df51",
    //   "queued": 3,
    //   "status": "QUEUED",
    //   "subject": "October product update",
    //   "template_id": "9c360595-1b04-500b-ac1e-20f6330c4995",
    //   "template_version": 3,
    //   "to": "To",
    //   "variant_key": "Variant key",
    //   "workflow_id": "f56c2ab8-e37f-57c9-80ee-b3c2d7d0cbe7"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "campaign_id": "94cb11b7-4223-5b33-bb8d-320012c437a0",
  "campaign_name": "Campaign name",
  "channel": "EMAIL",
  "clicked_at": "2026-11-05T16:00:00Z",
  "clicked_link": "Clicked link",
  "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
  "contact_name": "Contact name",
  "created_at": "2026-11-05T16:00:00Z",
  "enrolment_id": "a8314e09-6da8-5b0a-8ac0-8ffd966cf280",
  "error": "Error",
  "events": [
    {
      "at": "2026-11-05T16:00:00Z",
      "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
      "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
      "send_id": "a28a1db3-cee2-594e-adbd-25b5f5f34365",
      "type": "QUEUED"
    }
  ],
  "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
  "opened_at": "2026-11-05T16:00:00Z",
  "provider": "SES",
  "provider_message_id": "ext_e7e895df51",
  "queued": 3,
  "status": "QUEUED",
  "subject": "October product update",
  "template_id": "9c360595-1b04-500b-ac1e-20f6330c4995",
  "template_version": 3,
  "to": "To",
  "variant_key": "Variant key",
  "workflow_id": "f56c2ab8-e37f-57c9-80ee-b3c2d7d0cbe7"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        campaign_id: Some("94cb11b7-4223-5b33-bb8d-320012c437a0".to_string()),campaign_name: Some("Campaign name".to_string()),channel: serde_json::from_value::<Channel>(serde_json::Value::String("EMAIL".to_string())).unwrap_or_default(),clicked_at: Some("2026-11-05T16:00:00Z".to_string()),clicked_link: Some("Clicked link".to_string()),contact_id: "c891a339-996d-56b9-a022-244c570f48c9".to_string(),contact_name: Some("Contact name".to_string()),created_at: "2026-11-05T16:00:00Z".to_string(),enrolment_id: Some("a8314e09-6da8-5b0a-8ac0-8ffd966cf280".to_string()),error: Some("Error".to_string()),events: Some(vec![serde_json::from_value::<MessageEvent>(serde_json::json!({"at":"2026-11-05T16:00:00Z","contact_id":"c891a339-996d-56b9-a022-244c570f48c9","id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76","send_id":"a28a1db3-cee2-594e-adbd-25b5f5f34365","type":"QUEUED"})).unwrap_or_default()]),id: "3eb009b7-a99d-5a4c-956a-99e11d8dbb76".to_string(),opened_at: Some("2026-11-05T16:00:00Z".to_string()),provider: Some(serde_json::from_value::<Provider>(serde_json::Value::String("SES".to_string())).unwrap_or_default()),provider_message_id: Some("ext_e7e895df51".to_string()),queued: Some(3),status: serde_json::from_value::<SendStatus>(serde_json::Value::String("QUEUED".to_string())).unwrap_or_default(),subject: Some("October product update".to_string()),template_id: Some("9c360595-1b04-500b-ac1e-20f6330c4995".to_string()),template_version: Some(3),to: Some("To".to_string()),variant_key: Some("Variant key".to_string()),workflow_id: Some("f56c2ab8-e37f-57c9-80ee-b3c2d7d0cbe7".to_string()),
    })
}
