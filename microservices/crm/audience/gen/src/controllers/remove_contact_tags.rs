// User-owned controller for handler 'remove_contact_tags'.

use crate::handlers::remove_contact_tags::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RemoveContactTagsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "attribution": {
    //     "landing_page": "Landing page",
    //     "referrer": "Referrer",
    //     "utm_campaign": "Utm campaign",
    //     "utm_content": "Utm content",
    //     "utm_medium": "Utm medium",
    //     "utm_source": "google",
    //     "utm_term": "Utm term"
    //   },
    //   "country_code": "DE",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "custom_fields": {
    //     "industry": "Industry",
    //     "plan": "Plan",
    //     "renewal_date": "2026-11-25",
    //     "seats": 3,
    //     "team_size": "Team size"
    //   },
    //   "email": "priya.natarajan@northwindtraders.example",
    //   "first_name": "Priya",
    //   "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
    //   "job_title": "Job title",
    //   "last_activity_at": "2026-11-05T16:00:00Z",
    //   "last_name": "Natarajan",
    //   "list_ids": [
    //     "aa8fda45-3921-50e9-ba9c-b808dad9f340"
    //   ],
    //   "list_names": [
    //     "List names"
    //   ],
    //   "message_stats": {
    //     "hard_bounces": 3,
    //     "last_click_at": "2026-11-05T16:00:00Z",
    //     "last_open_at": "2026-11-05T16:00:00Z",
    //     "sent": 3,
    //     "soft_bounces": 3
    //   },
    //   "organisation_id": "82a361eb-96c9-56a2-8291-33ba572abba9",
    //   "organisation_name": "Organisation name",
    //   "phone": "+49 30 1000000",
    //   "score": 3,
    //   "sms_consent": false,
    //   "sms_consent_updated_at": "2026-11-05T16:00:00Z",
    //   "source": "Source",
    //   "status": "SUBSCRIBED",
    //   "tags": [
    //     "Tags"
    //   ],
    //   "timezone": "Europe/Berlin",
    //   "tracking_optout": false,
    //   "updated_at": "2026-11-05T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "attribution": {
    "landing_page": "Landing page",
    "referrer": "Referrer",
    "utm_campaign": "Utm campaign",
    "utm_content": "Utm content",
    "utm_medium": "Utm medium",
    "utm_source": "google",
    "utm_term": "Utm term"
  },
  "country_code": "DE",
  "created_at": "2026-11-05T16:00:00Z",
  "custom_fields": {
    "industry": "Industry",
    "plan": "Plan",
    "renewal_date": "2026-11-25",
    "seats": 3,
    "team_size": "Team size"
  },
  "email": "priya.natarajan@northwindtraders.example",
  "first_name": "Priya",
  "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
  "job_title": "Job title",
  "last_activity_at": "2026-11-05T16:00:00Z",
  "last_name": "Natarajan",
  "list_ids": [
    "aa8fda45-3921-50e9-ba9c-b808dad9f340"
  ],
  "list_names": [
    "List names"
  ],
  "message_stats": {
    "hard_bounces": 3,
    "last_click_at": "2026-11-05T16:00:00Z",
    "last_open_at": "2026-11-05T16:00:00Z",
    "sent": 3,
    "soft_bounces": 3
  },
  "organisation_id": "82a361eb-96c9-56a2-8291-33ba572abba9",
  "organisation_name": "Organisation name",
  "phone": "+49 30 1000000",
  "score": 3,
  "sms_consent": false,
  "sms_consent_updated_at": "2026-11-05T16:00:00Z",
  "source": "Source",
  "status": "SUBSCRIBED",
  "tags": [
    "Tags"
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
        attribution: Some(serde_json::from_str::<serde_json::Value>(r#"{"landing_page":"Landing page","referrer":"Referrer","utm_campaign":"Utm campaign","utm_content":"Utm content","utm_medium":"Utm medium","utm_source":"google","utm_term":"Utm term"}"#).unwrap_or(serde_json::Value::Null)),country_code: Some("DE".to_string()),created_at: "2026-11-05T16:00:00Z".to_string(),custom_fields: Some(serde_json::from_str::<serde_json::Value>(r#"{"industry":"Industry","plan":"Plan","renewal_date":"2026-11-25","seats":3,"team_size":"Team size"}"#).unwrap_or(serde_json::Value::Null)),email: "priya.natarajan@northwindtraders.example".to_string(),first_name: Some("Priya".to_string()),id: "44f5daeb-faac-5a53-81fb-ac8b5f267d2c".to_string(),job_title: Some("Job title".to_string()),last_activity_at: Some("2026-11-05T16:00:00Z".to_string()),last_name: Some("Natarajan".to_string()),list_ids: Some(vec!["aa8fda45-3921-50e9-ba9c-b808dad9f340".to_string()]),list_names: Some(vec!["List names".to_string()]),message_stats: Some(serde_json::from_str::<serde_json::Value>(r#"{"hard_bounces":3,"last_click_at":"2026-11-05T16:00:00Z","last_open_at":"2026-11-05T16:00:00Z","sent":3,"soft_bounces":3}"#).unwrap_or(serde_json::Value::Null)),organisation_id: Some("82a361eb-96c9-56a2-8291-33ba572abba9".to_string()),organisation_name: Some("Organisation name".to_string()),phone: Some("+49 30 1000000".to_string()),score: Some(3),sms_consent: Some(false),sms_consent_updated_at: Some("2026-11-05T16:00:00Z".to_string()),source: Some("Source".to_string()),status: "SUBSCRIBED".to_string(),tags: Some(vec!["Tags".to_string()]),timezone: Some("Europe/Berlin".to_string()),tracking_optout: Some(false),updated_at: Some("2026-11-05T16:00:00Z".to_string()),
    })
}
