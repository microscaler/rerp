// User-owned controller for handler 'update_contact'.

use crate::handlers::update_contact::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateContactController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "attribution": {
    //     "utm_campaign": "q4-webinars",
    //     "utm_medium": "cpc",
    //     "utm_source": "google"
    //   },
    //   "country_code": "DE",
    //   "created_at": "2026-10-31T16:00:00Z",
    //   "custom_fields": {
    //     "industry": "Logistics",
    //     "plan": "Free",
    //     "renewal_date": "2026-11-25",
    //     "seats": 5,
    //     "team_size": "1-10"
    //   },
    //   "email": "priya.natarajan@northwindtraders.example",
    //   "first_name": "Priya",
    //   "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //   "job_title": "Head of Operations",
    //   "last_activity_at": "2026-11-05T15:00:00Z",
    //   "last_name": "Natarajan",
    //   "list_ids": [
    //     "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //     "f84269dd-7dba-5a82-ad63-986af03f39c1"
    //   ],
    //   "list_names": [
    //     "Newsletter",
    //     "Customers"
    //   ],
    //   "message_stats": {
    //     "hard_bounces": 0,
    //     "last_click_at": "2026-11-04T16:00:00Z",
    //     "last_open_at": "2026-11-05T14:00:00Z",
    //     "sent": 12,
    //     "soft_bounces": 0
    //   },
    //   "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //   "organisation_name": "Northwind Traders",
    //   "phone": "+49 30 1000000",
    //   "score": 85,
    //   "sms_consent": true,
    //   "sms_consent_updated_at": "2026-09-26T16:00:00Z",
    //   "source": "form:website-footer",
    //   "status": "SUBSCRIBED",
    //   "tags": [
    //     "customer",
    //     "enterprise"
    //   ],
    //   "timezone": "Europe/Berlin",
    //   "tracking_optout": false,
    //   "updated_at": "2026-11-05T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "attribution": {
    "utm_campaign": "q4-webinars",
    "utm_medium": "cpc",
    "utm_source": "google"
  },
  "country_code": "DE",
  "created_at": "2026-10-31T16:00:00Z",
  "custom_fields": {
    "industry": "Logistics",
    "plan": "Free",
    "renewal_date": "2026-11-25",
    "seats": 5,
    "team_size": "1-10"
  },
  "email": "priya.natarajan@northwindtraders.example",
  "first_name": "Priya",
  "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
  "job_title": "Head of Operations",
  "last_activity_at": "2026-11-05T15:00:00Z",
  "last_name": "Natarajan",
  "list_ids": [
    "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    "f84269dd-7dba-5a82-ad63-986af03f39c1"
  ],
  "list_names": [
    "Newsletter",
    "Customers"
  ],
  "message_stats": {
    "hard_bounces": 0,
    "last_click_at": "2026-11-04T16:00:00Z",
    "last_open_at": "2026-11-05T14:00:00Z",
    "sent": 12,
    "soft_bounces": 0
  },
  "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
  "organisation_name": "Northwind Traders",
  "phone": "+49 30 1000000",
  "score": 85,
  "sms_consent": true,
  "sms_consent_updated_at": "2026-09-26T16:00:00Z",
  "source": "form:website-footer",
  "status": "SUBSCRIBED",
  "tags": [
    "customer",
    "enterprise"
  ],
  "timezone": "Europe/Berlin",
  "tracking_optout": false,
  "updated_at": "2026-11-05T16:00:00Z"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        attribution: Some(serde_json::from_str::<serde_json::Value>(r#"{"utm_campaign":"q4-webinars","utm_medium":"cpc","utm_source":"google"}"#).unwrap_or(serde_json::Value::Null)),country_code: Some("DE".to_string()),created_at: "2026-10-31T16:00:00Z".to_string(),custom_fields: Some(serde_json::from_str::<serde_json::Value>(r#"{"industry":"Logistics","plan":"Free","renewal_date":"2026-11-25","seats":5,"team_size":"1-10"}"#).unwrap_or(serde_json::Value::Null)),email: "priya.natarajan@northwindtraders.example".to_string(),first_name: Some("Priya".to_string()),id: "af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string(),job_title: Some("Head of Operations".to_string()),last_activity_at: Some("2026-11-05T15:00:00Z".to_string()),last_name: Some("Natarajan".to_string()),list_ids: Some(vec!["09f5b2f9-2adc-5edd-a6fd-c20675680671".to_string(), "f84269dd-7dba-5a82-ad63-986af03f39c1".to_string()]),list_names: Some(vec!["Newsletter".to_string(), "Customers".to_string()]),message_stats: Some(serde_json::from_str::<serde_json::Value>(r#"{"hard_bounces":0,"last_click_at":"2026-11-04T16:00:00Z","last_open_at":"2026-11-05T14:00:00Z","sent":12,"soft_bounces":0}"#).unwrap_or(serde_json::Value::Null)),organisation_id: Some("d7a2e2b8-61b1-5687-8aee-0ead8943ba3b".to_string()),organisation_name: Some("Northwind Traders".to_string()),phone: Some("+49 30 1000000".to_string()),score: Some(85),sms_consent: Some(true),sms_consent_updated_at: Some("2026-09-26T16:00:00Z".to_string()),source: Some("form:website-footer".to_string()),status: "SUBSCRIBED".to_string(),tags: Some(vec!["customer".to_string(), "enterprise".to_string()]),timezone: Some("Europe/Berlin".to_string()),tracking_optout: Some(false),updated_at: Some("2026-11-05T16:00:00Z".to_string()),
    })
}
