// User-owned controller for handler 'get_contact'.

use crate::handlers::get_contact::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetContactController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "create_date": "2026-11-05T16:00:00Z",
    //   "custom_fields": {
    //     "industry": "Logistics",
    //     "plan": "Free",
    //     "renewal_date": null,
    //     "seats": 5,
    //     "team_size": "1-10"
    //   },
    //   "email": "priya.natarajan@northwindtraders.example",
    //   "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //   "last_activity_at": "2026-11-05T15:00:00Z",
    //   "name": "Northwind Traders",
    //   "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //   "phone": "+49 30 1000000",
    //   "score": 85,
    //   "tags": [
    //     "customer",
    //     "enterprise"
    //   ],
    //   "timezone": "Europe/Berlin"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "create_date": "2026-11-05T16:00:00Z",
  "custom_fields": {
    "industry": "Logistics",
    "plan": "Free",
    "renewal_date": null,
    "seats": 5,
    "team_size": "1-10"
  },
  "email": "priya.natarajan@northwindtraders.example",
  "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
  "last_activity_at": "2026-11-05T15:00:00Z",
  "name": "Northwind Traders",
  "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
  "phone": "+49 30 1000000",
  "score": 85,
  "tags": [
    "customer",
    "enterprise"
  ],
  "timezone": "Europe/Berlin"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        active: Some(true),child_contact_ids: Some(vec![]),city: Some("example".to_string()),color: Some(42),country_id: Some("example".to_string()),create_date: "2026-11-05T16:00:00Z".to_string(),custom_fields: Some(serde_json::from_str::<serde_json::Value>(r#"{"industry":"Logistics","plan":"Free","renewal_date":null,"seats":5,"team_size":"1-10"}"#).unwrap_or(serde_json::Value::Null)),department: Some("example".to_string()),email: Some("priya.natarajan@northwindtraders.example".to_string()),function: Some("example".to_string()),id: "af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string(),is_company: Some(true),last_activity_at: Some("2026-11-05T15:00:00Z".to_string()),lead_count: Some(42),mobile: Some("example".to_string()),name: "Northwind Traders".to_string(),opportunity_count: Some(42),organisation_id: Some("d7a2e2b8-61b1-5687-8aee-0ead8943ba3b".to_string()),parent_id: Some("example".to_string()),phone: Some("+49 30 1000000".to_string()),score: Some(85),state_id: Some("example".to_string()),street: Some("example".to_string()),street2: Some("example".to_string()),tags: Some(vec!["customer".to_string(), "enterprise".to_string()]),timezone: Some("Europe/Berlin".to_string()),title: Some("example".to_string()),user_ids: Some(vec![]),write_date: Some("example".to_string()),zip: Some("example".to_string()),
    })
}
