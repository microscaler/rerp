// User-owned controller for handler 'change_deal_stage'.

use crate::handlers::change_deal_stage::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealContact;
#[allow(unused_imports)]
use crate::handlers::types::DealStatus;

#[handler(ChangeDealStageController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contacts": [
    //     {
    //       "contact_id": "cab01328-10bf-5c9a-9c3a-5c2c57018288"
    //     }
    //   ],
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "created_by": "Created by",
    //   "currency": "EUR",
    //   "custom_fields": {},
    //   "days_in_stage": 3,
    //   "description": "Example",
    //   "expected_close_date": "2026-11-25",
    //   "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
    //   "last_activity_at": "2026-11-05T16:00:00Z",
    //   "lost_reason": "Lost reason",
    //   "next_task_at": "2026-11-05T16:00:00Z",
    //   "open_task_count": 3,
    //   "organisation_id": "f81747aa-a909-54a1-bd00-e1888a20c4dc",
    //   "organisation_name": "Organisation name",
    //   "owner_id": "alex",
    //   "owner_name": "Owner name",
    //   "pipeline_id": "32dd9c82-6ef8-54b5-816b-7f32b5de5050",
    //   "pipeline_name": "Pipeline name",
    //   "primary_contact_id": "8eb8743a-71f2-53a5-a172-92fda4d35c22",
    //   "probability": 3,
    //   "source": "Source",
    //   "stage_entered_at": "2026-11-05T16:00:00Z",
    //   "stage_id": "fd284c39-47ef-593e-aa63-650247d40d35",
    //   "stage_name": "Stage name",
    //   "status": "OPEN",
    //   "status_changed_at": "2026-11-05T16:00:00Z",
    //   "tag_ids": [
    //     "b17c2272-ee36-5602-8352-7cd0f3c66180"
    //   ],
    //   "title": "Northwind · 48 → 120 seats",
    //   "updated_at": "2026-11-05T16:00:00Z",
    //   "value": "1250.00",
    //   "weighted_value": "1250.00"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "contacts": [
    {
      "contact_id": "cab01328-10bf-5c9a-9c3a-5c2c57018288"
    }
  ],
  "created_at": "2026-11-05T16:00:00Z",
  "created_by": "Created by",
  "currency": "EUR",
  "custom_fields": {},
  "days_in_stage": 3,
  "description": "Example",
  "expected_close_date": "2026-11-25",
  "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
  "last_activity_at": "2026-11-05T16:00:00Z",
  "lost_reason": "Lost reason",
  "next_task_at": "2026-11-05T16:00:00Z",
  "open_task_count": 3,
  "organisation_id": "f81747aa-a909-54a1-bd00-e1888a20c4dc",
  "organisation_name": "Organisation name",
  "owner_id": "alex",
  "owner_name": "Owner name",
  "pipeline_id": "32dd9c82-6ef8-54b5-816b-7f32b5de5050",
  "pipeline_name": "Pipeline name",
  "primary_contact_id": "8eb8743a-71f2-53a5-a172-92fda4d35c22",
  "probability": 3,
  "source": "Source",
  "stage_entered_at": "2026-11-05T16:00:00Z",
  "stage_id": "fd284c39-47ef-593e-aa63-650247d40d35",
  "stage_name": "Stage name",
  "status": "OPEN",
  "status_changed_at": "2026-11-05T16:00:00Z",
  "tag_ids": [
    "b17c2272-ee36-5602-8352-7cd0f3c66180"
  ],
  "title": "Northwind · 48 → 120 seats",
  "updated_at": "2026-11-05T16:00:00Z",
  "value": "1250.00",
  "weighted_value": "1250.00"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        contacts: Some(vec![serde_json::from_value::<DealContact>(
            serde_json::json!({"contact_id":"cab01328-10bf-5c9a-9c3a-5c2c57018288"}),
        )
        .unwrap_or_default()]),
        created_at: "2026-11-05T16:00:00Z".to_string(),
        created_by: Some("Created by".to_string()),
        currency: "EUR".to_string(),
        custom_fields: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        days_in_stage: Some(3),
        description: Some("Example".to_string()),
        expected_close_date: Some("2026-11-25".to_string()),
        id: "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26".to_string(),
        last_activity_at: Some("2026-11-05T16:00:00Z".to_string()),
        lost_reason: Some("Lost reason".to_string()),
        next_task_at: Some("2026-11-05T16:00:00Z".to_string()),
        open_task_count: Some(3),
        organisation_id: Some("f81747aa-a909-54a1-bd00-e1888a20c4dc".to_string()),
        organisation_name: Some("Organisation name".to_string()),
        owner_id: Some("alex".to_string()),
        owner_name: Some("Owner name".to_string()),
        pipeline_id: "32dd9c82-6ef8-54b5-816b-7f32b5de5050".to_string(),
        pipeline_name: Some("Pipeline name".to_string()),
        primary_contact_id: Some("8eb8743a-71f2-53a5-a172-92fda4d35c22".to_string()),
        probability: Some(3),
        source: Some("Source".to_string()),
        stage_entered_at: Some("2026-11-05T16:00:00Z".to_string()),
        stage_id: "fd284c39-47ef-593e-aa63-650247d40d35".to_string(),
        stage_name: Some("Stage name".to_string()),
        status: serde_json::from_value::<DealStatus>(serde_json::Value::String("OPEN".to_string()))
            .unwrap_or_default(),
        status_changed_at: Some("2026-11-05T16:00:00Z".to_string()),
        tag_ids: Some(vec!["b17c2272-ee36-5602-8352-7cd0f3c66180".to_string()]),
        title: "Northwind · 48 → 120 seats".to_string(),
        updated_at: Some("2026-11-05T16:00:00Z".to_string()),
        value: "1250.00".to_string(),
        weighted_value: Some("1250.00".to_string()),
    })
}
