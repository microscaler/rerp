// User-owned controller for handler 'update_deal'.

use crate::handlers::update_deal::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealContact;
#[allow(unused_imports)]
use crate::handlers::types::DealStatus;

#[handler(UpdateDealController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contacts": [
    //     {
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "name": "Priya Natarajan",
    //       "role": "Primary contact"
    //     }
    //   ],
    //   "created_at": "2026-10-13T16:00:00Z",
    //   "currency": "EUR",
    //   "custom_fields": {
    //     "seats": 10,
    //     "term_months": "12"
    //   },
    //   "days_in_stage": 3,
    //   "expected_close_date": "2026-11-25",
    //   "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //   "last_activity_at": "2026-11-02T16:00:00Z",
    //   "next_task_at": "2026-11-06T16:00:00Z",
    //   "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //   "organisation_name": "Northwind Traders",
    //   "owner_id": "alex",
    //   "owner_name": "Alex Morgan",
    //   "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //   "probability": 50,
    //   "source": "customer expansion",
    //   "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //   "stage_name": "Proposal",
    //   "status": "OPEN",
    //   "title": "Northwind · 48 → 120 seats",
    //   "updated_at": "2026-11-02T16:00:00Z",
    //   "value": "38400.00",
    //   "weighted_value": "19200.00"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "contacts": [
    {
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "name": "Priya Natarajan",
      "role": "Primary contact"
    }
  ],
  "created_at": "2026-10-13T16:00:00Z",
  "currency": "EUR",
  "custom_fields": {
    "seats": 10,
    "term_months": "12"
  },
  "days_in_stage": 3,
  "expected_close_date": "2026-11-25",
  "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
  "last_activity_at": "2026-11-02T16:00:00Z",
  "next_task_at": "2026-11-06T16:00:00Z",
  "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
  "organisation_name": "Northwind Traders",
  "owner_id": "alex",
  "owner_name": "Alex Morgan",
  "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
  "probability": 50,
  "source": "customer expansion",
  "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
  "stage_name": "Proposal",
  "status": "OPEN",
  "title": "Northwind · 48 → 120 seats",
  "updated_at": "2026-11-02T16:00:00Z",
  "value": "38400.00",
  "weighted_value": "19200.00"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        contacts: Some(vec![serde_json::from_value::<DealContact>(serde_json::json!({"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","name":"Priya Natarajan","role":"Primary contact"})).unwrap_or_default()]),created_at: "2026-10-13T16:00:00Z".to_string(),created_by: Some("example".to_string()),currency: "EUR".to_string(),custom_fields: Some(serde_json::from_str::<serde_json::Value>(r#"{"seats":10,"term_months":"12"}"#).unwrap_or(serde_json::Value::Null)),days_in_stage: Some(3),description: Some("example".to_string()),expected_close_date: Some("2026-11-25".to_string()),id: "63a6661f-5860-50ed-8697-f28d29a3917f".to_string(),last_activity_at: Some("2026-11-02T16:00:00Z".to_string()),lost_reason: Some("example".to_string()),next_task_at: Some("2026-11-06T16:00:00Z".to_string()),open_task_count: Some(42),organisation_id: Some("d7a2e2b8-61b1-5687-8aee-0ead8943ba3b".to_string()),organisation_name: Some("Northwind Traders".to_string()),owner_id: Some("alex".to_string()),owner_name: Some("Alex Morgan".to_string()),pipeline_id: "d9a2022e-96ff-53a3-a94d-2b169424fd31".to_string(),pipeline_name: Some("example".to_string()),primary_contact_id: Some("example".to_string()),probability: Some(50),source: Some("customer expansion".to_string()),stage_entered_at: Some("example".to_string()),stage_id: "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5".to_string(),stage_name: Some("Proposal".to_string()),status: serde_json::from_value::<DealStatus>(serde_json::Value::String("OPEN".to_string())).unwrap_or_default(),status_changed_at: Some("example".to_string()),tag_ids: Some(vec![]),title: "Northwind · 48 → 120 seats".to_string(),updated_at: Some("2026-11-02T16:00:00Z".to_string()),value: "38400.00".to_string(),weighted_value: Some("19200.00".to_string()),
    })
}
