// User-owned controller for handler 'add_deal_contact'.

use crate::handlers::add_deal_contact::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealContact;
#[allow(unused_imports)]
use crate::handlers::types::DealStatus;

#[handler(AddDealContactController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contact_id": "93f1db66-d1af-580b-b546-6a13f1774d12",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "currency": "EUR",
    //   "email": "marcus.reid@adventureworks.example",
    //   "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
    //   "is_primary": false,
    //   "name": "Marcus Reid",
    //   "pipeline_id": "32dd9c82-6ef8-54b5-816b-7f32b5de5050",
    //   "role": "Champion",
    //   "stage_id": "fd284c39-47ef-593e-aa63-650247d40d35",
    //   "status": "OPEN",
    //   "title": "Northwind · 48 → 120 seats",
    //   "value": "1250.00"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "contact_id": "93f1db66-d1af-580b-b546-6a13f1774d12",
  "created_at": "2026-11-05T16:00:00Z",
  "currency": "EUR",
  "email": "marcus.reid@adventureworks.example",
  "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
  "is_primary": false,
  "name": "Marcus Reid",
  "pipeline_id": "32dd9c82-6ef8-54b5-816b-7f32b5de5050",
  "role": "Champion",
  "stage_id": "fd284c39-47ef-593e-aa63-650247d40d35",
  "status": "OPEN",
  "title": "Northwind · 48 → 120 seats",
  "value": "1250.00"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        contact_id: Some("93f1db66-d1af-580b-b546-6a13f1774d12".to_string()),
        contacts: Some(vec![]),
        created_at: "2026-11-05T16:00:00Z".to_string(),
        created_by: Some("example".to_string()),
        currency: "EUR".to_string(),
        custom_fields: Some(Default::default()),
        days_in_stage: Some(42),
        description: Some("example".to_string()),
        email: Some("marcus.reid@adventureworks.example".to_string()),
        expected_close_date: Some("example".to_string()),
        id: "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26".to_string(),
        is_primary: Some(false),
        last_activity_at: Some("example".to_string()),
        lost_reason: Some("example".to_string()),
        name: Some("Marcus Reid".to_string()),
        next_task_at: Some("example".to_string()),
        open_task_count: Some(42),
        organisation_id: Some("example".to_string()),
        organisation_name: Some("example".to_string()),
        owner_id: Some("example".to_string()),
        owner_name: Some("example".to_string()),
        pipeline_id: "32dd9c82-6ef8-54b5-816b-7f32b5de5050".to_string(),
        pipeline_name: Some("example".to_string()),
        primary_contact_id: Some("example".to_string()),
        probability: Some(42),
        role: Some("Champion".to_string()),
        source: Some("example".to_string()),
        stage_entered_at: Some("example".to_string()),
        stage_id: "fd284c39-47ef-593e-aa63-650247d40d35".to_string(),
        stage_name: Some("example".to_string()),
        status: serde_json::from_value::<DealStatus>(serde_json::Value::String("OPEN".to_string()))
            .unwrap_or_default(),
        status_changed_at: Some("example".to_string()),
        tag_ids: Some(vec![]),
        title: "Northwind · 48 → 120 seats".to_string(),
        updated_at: Some("example".to_string()),
        value: "1250.00".to_string(),
        weighted_value: Some("example".to_string()),
    })
}
