// User-owned controller for handler 'update_task'.

use crate::handlers::update_task::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AutomationHook;
#[allow(unused_imports)]
use crate::handlers::types::RelatedKind;
#[allow(unused_imports)]
use crate::handlers::types::Reminder;
#[allow(unused_imports)]
use crate::handlers::types::Sentiment;
#[allow(unused_imports)]
use crate::handlers::types::TaskStatus;

#[handler(UpdateTaskController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "completed_at": "2026-11-05T16:00:00Z",
    //   "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //   "contact_name": "Priya Natarajan",
    //   "created_at": "2026-10-31T16:00:00Z",
    //   "created_by": "alex",
    //   "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //   "deal_title": "Northwind · 48 → 120 seats",
    //   "due_at": "2026-11-05T16:00:00Z",
    //   "duration_minutes": 15,
    //   "id": "89a25b3e-8a9c-514a-b373-cd9ef34c7cbc",
    //   "note": "Priya wants SSO before expanding seats",
    //   "outcome": "Outcome",
    //   "overdue": true,
    //   "owner_id": "alex",
    //   "owner_name": "Alex Morgan",
    //   "related_id": "57d59745-33ec-5dc8-9adc-cc139255687c",
    //   "related_kind": "CONTACT",
    //   "status": "OPEN",
    //   "title": "Call about SSO requirements",
    //   "type": "Call",
    //   "type_id": "4eec3526-5869-5e76-820f-ac2358c6b899"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "completed_at": "2026-11-05T16:00:00Z",
  "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
  "contact_name": "Priya Natarajan",
  "created_at": "2026-10-31T16:00:00Z",
  "created_by": "alex",
  "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
  "deal_title": "Northwind · 48 → 120 seats",
  "due_at": "2026-11-05T16:00:00Z",
  "duration_minutes": 15,
  "id": "89a25b3e-8a9c-514a-b373-cd9ef34c7cbc",
  "note": "Priya wants SSO before expanding seats",
  "outcome": "Outcome",
  "overdue": true,
  "owner_id": "alex",
  "owner_name": "Alex Morgan",
  "related_id": "57d59745-33ec-5dc8-9adc-cc139255687c",
  "related_kind": "CONTACT",
  "status": "OPEN",
  "title": "Call about SSO requirements",
  "type": "Call",
  "type_id": "4eec3526-5869-5e76-820f-ac2358c6b899"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        automation: Some(Default::default()),
        completed_at: Some("2026-11-05T16:00:00Z".to_string()),
        completed_by: Some("example".to_string()),
        contact_id: Some("af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string()),
        contact_name: Some("Priya Natarajan".to_string()),
        created_at: "2026-10-31T16:00:00Z".to_string(),
        created_by: Some("alex".to_string()),
        deal_id: Some("63a6661f-5860-50ed-8697-f28d29a3917f".to_string()),
        deal_title: Some("Northwind · 48 → 120 seats".to_string()),
        due_at: Some("2026-11-05T16:00:00Z".to_string()),
        duration_minutes: Some(15),
        id: "89a25b3e-8a9c-514a-b373-cd9ef34c7cbc".to_string(),
        note: Some("Priya wants SSO before expanding seats".to_string()),
        outcome: Some("Outcome".to_string()),
        outcome_id: Some("example".to_string()),
        outcome_sentiment: Some(Default::default()),
        overdue: Some(true),
        owner_id: Some("alex".to_string()),
        owner_name: Some("Alex Morgan".to_string()),
        related_id: "57d59745-33ec-5dc8-9adc-cc139255687c".to_string(),
        related_kind: serde_json::from_value::<RelatedKind>(serde_json::Value::String(
            "CONTACT".to_string(),
        ))
        .unwrap_or_default(),
        reminders: Some(vec![]),
        status: serde_json::from_value::<TaskStatus>(serde_json::Value::String("OPEN".to_string()))
            .unwrap_or_default(),
        title: "Call about SSO requirements".to_string(),
        r#type: Some("Call".to_string()),
        type_id: "4eec3526-5869-5e76-820f-ac2358c6b899".to_string(),
        updated_at: Some("example".to_string()),
    })
}
