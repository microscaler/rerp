// User-owned controller for handler 'remove_contact_tags'.

use crate::handlers::remove_contact_tags::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RemoveContactTagsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "active": true,
    //   "child_contact_ids": [
    //     "8da3ada2-4577-5570-8894-0631a83d0667"
    //   ],
    //   "city": "City",
    //   "color": 3,
    //   "country_id": "805a89a0-e9a0-5bff-b298-223bac74e954",
    //   "create_date": "2026-11-05T16:00:00Z",
    //   "custom_fields": {},
    //   "department": "Department",
    //   "email": "priya.natarajan@northwindtraders.example",
    //   "function": "Function",
    //   "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
    //   "is_company": true,
    //   "last_activity_at": "2026-11-05T16:00:00Z",
    //   "lead_count": 3,
    //   "mobile": "Mobile",
    //   "name": "Northwind Traders",
    //   "opportunity_count": 3,
    //   "organisation_id": "82a361eb-96c9-56a2-8291-33ba572abba9",
    //   "parent_id": "f72b0f27-57e7-5999-986e-0069569b31a7",
    //   "phone": "+49 30 1000000",
    //   "score": 3,
    //   "state_id": "e428ab8b-c4da-5422-b94c-d2c2e53afd04",
    //   "street": "Street",
    //   "street2": "Street2",
    //   "tags": [
    //     "Tags"
    //   ],
    //   "timezone": "Europe/Berlin",
    //   "title": "MR",
    //   "user_ids": [
    //     "User ids"
    //   ],
    //   "write_date": "2026-11-05T16:00:00Z",
    //   "zip": "Zip"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "active": true,
  "child_contact_ids": [
    "8da3ada2-4577-5570-8894-0631a83d0667"
  ],
  "city": "City",
  "color": 3,
  "country_id": "805a89a0-e9a0-5bff-b298-223bac74e954",
  "create_date": "2026-11-05T16:00:00Z",
  "custom_fields": {},
  "department": "Department",
  "email": "priya.natarajan@northwindtraders.example",
  "function": "Function",
  "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
  "is_company": true,
  "last_activity_at": "2026-11-05T16:00:00Z",
  "lead_count": 3,
  "mobile": "Mobile",
  "name": "Northwind Traders",
  "opportunity_count": 3,
  "organisation_id": "82a361eb-96c9-56a2-8291-33ba572abba9",
  "parent_id": "f72b0f27-57e7-5999-986e-0069569b31a7",
  "phone": "+49 30 1000000",
  "score": 3,
  "state_id": "e428ab8b-c4da-5422-b94c-d2c2e53afd04",
  "street": "Street",
  "street2": "Street2",
  "tags": [
    "Tags"
  ],
  "timezone": "Europe/Berlin",
  "title": "MR",
  "user_ids": [
    "User ids"
  ],
  "write_date": "2026-11-05T16:00:00Z",
  "zip": "Zip"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        active: Some(true),
        child_contact_ids: Some(vec!["8da3ada2-4577-5570-8894-0631a83d0667".to_string()]),
        city: Some("City".to_string()),
        color: Some(3),
        country_id: Some("805a89a0-e9a0-5bff-b298-223bac74e954".to_string()),
        create_date: "2026-11-05T16:00:00Z".to_string(),
        custom_fields: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        department: Some("Department".to_string()),
        email: Some("priya.natarajan@northwindtraders.example".to_string()),
        function: Some("Function".to_string()),
        id: "44f5daeb-faac-5a53-81fb-ac8b5f267d2c".to_string(),
        is_company: Some(true),
        last_activity_at: Some("2026-11-05T16:00:00Z".to_string()),
        lead_count: Some(3),
        mobile: Some("Mobile".to_string()),
        name: "Northwind Traders".to_string(),
        opportunity_count: Some(3),
        organisation_id: Some("82a361eb-96c9-56a2-8291-33ba572abba9".to_string()),
        parent_id: Some("f72b0f27-57e7-5999-986e-0069569b31a7".to_string()),
        phone: Some("+49 30 1000000".to_string()),
        score: Some(3),
        state_id: Some("e428ab8b-c4da-5422-b94c-d2c2e53afd04".to_string()),
        street: Some("Street".to_string()),
        street2: Some("Street2".to_string()),
        tags: Some(vec!["Tags".to_string()]),
        timezone: Some("Europe/Berlin".to_string()),
        title: Some("MR".to_string()),
        user_ids: Some(vec!["User ids".to_string()]),
        write_date: Some("2026-11-05T16:00:00Z".to_string()),
        zip: Some("Zip".to_string()),
    })
}
