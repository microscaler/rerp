// User-owned controller for handler 'list_contacts'.

use crate::handlers::list_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Contact;

#[handler(ListContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "create_date": "2026-11-05T16:00:00Z",
    //       "custom_fields": {
    //         "industry": "Logistics",
    //         "plan": "Free",
    //         "renewal_date": null,
    //         "seats": 5,
    //         "team_size": "1-10"
    //       },
    //       "email": "priya.natarajan@northwindtraders.example",
    //       "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "last_activity_at": "2026-11-05T15:00:00Z",
    //       "name": "Northwind Traders",
    //       "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //       "phone": "+49 30 1000000",
    //       "score": 85,
    //       "tags": [
    //         "customer",
    //         "enterprise"
    //       ],
    //       "timezone": "Europe/Berlin"
    //     },
    //     {
    //       "create_date": "2026-11-05T16:00:00Z",
    //       "custom_fields": {
    //         "industry": "Retail",
    //         "plan": "Team",
    //         "renewal_date": "2026-12-14",
    //         "seats": 8,
    //         "team_size": "11-50"
    //       },
    //       "email": "tom.okafor@fabrikam.example",
    //       "id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "last_activity_at": "2026-11-05T08:00:00Z",
    //       "name": "Northwind Traders",
    //       "organisation_id": "576a7630-a5b2-566c-ac00-caca59662cb0",
    //       "phone": "+49 30 1007919",
    //       "score": 79,
    //       "tags": [
    //         "trial"
    //       ],
    //       "timezone": "Europe/London"
    //     },
    //     {
    //       "create_date": "2026-11-05T16:00:00Z",
    //       "custom_fields": {
    //         "industry": "Finance",
    //         "plan": "Business",
    //         "renewal_date": "2026-12-23",
    //         "seats": 11,
    //         "team_size": "51+"
    //       },
    //       "email": "lena.fischer@contoso.example",
    //       "id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "last_activity_at": "2026-11-05T01:00:00Z",
    //       "name": "Northwind Traders",
    //       "organisation_id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
    //       "phone": "+49 30 1015838",
    //       "score": 84,
    //       "tags": [
    //         "waitlist"
    //       ],
    //       "timezone": "Africa/Johannesburg"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 30
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
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
    },
    {
      "create_date": "2026-11-05T16:00:00Z",
      "custom_fields": {
        "industry": "Retail",
        "plan": "Team",
        "renewal_date": "2026-12-14",
        "seats": 8,
        "team_size": "11-50"
      },
      "email": "tom.okafor@fabrikam.example",
      "id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "last_activity_at": "2026-11-05T08:00:00Z",
      "name": "Northwind Traders",
      "organisation_id": "576a7630-a5b2-566c-ac00-caca59662cb0",
      "phone": "+49 30 1007919",
      "score": 79,
      "tags": [
        "trial"
      ],
      "timezone": "Europe/London"
    },
    {
      "create_date": "2026-11-05T16:00:00Z",
      "custom_fields": {
        "industry": "Finance",
        "plan": "Business",
        "renewal_date": "2026-12-23",
        "seats": 11,
        "team_size": "51+"
      },
      "email": "lena.fischer@contoso.example",
      "id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "last_activity_at": "2026-11-05T01:00:00Z",
      "name": "Northwind Traders",
      "organisation_id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
      "phone": "+49 30 1015838",
      "score": 84,
      "tags": [
        "waitlist"
      ],
      "timezone": "Africa/Johannesburg"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 30
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Contact>(serde_json::json!({"create_date":"2026-11-05T16:00:00Z","custom_fields":{"industry":"Logistics","plan":"Free","renewal_date":null,"seats":5,"team_size":"1-10"},"email":"priya.natarajan@northwindtraders.example","id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","last_activity_at":"2026-11-05T15:00:00Z","name":"Northwind Traders","organisation_id":"d7a2e2b8-61b1-5687-8aee-0ead8943ba3b","phone":"+49 30 1000000","score":85,"tags":["customer","enterprise"],"timezone":"Europe/Berlin"})).unwrap_or_default(), serde_json::from_value::<Contact>(serde_json::json!({"create_date":"2026-11-05T16:00:00Z","custom_fields":{"industry":"Retail","plan":"Team","renewal_date":"2026-12-14","seats":8,"team_size":"11-50"},"email":"tom.okafor@fabrikam.example","id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","last_activity_at":"2026-11-05T08:00:00Z","name":"Northwind Traders","organisation_id":"576a7630-a5b2-566c-ac00-caca59662cb0","phone":"+49 30 1007919","score":79,"tags":["trial"],"timezone":"Europe/London"})).unwrap_or_default(), serde_json::from_value::<Contact>(serde_json::json!({"create_date":"2026-11-05T16:00:00Z","custom_fields":{"industry":"Finance","plan":"Business","renewal_date":"2026-12-23","seats":11,"team_size":"51+"},"email":"lena.fischer@contoso.example","id":"f25ace60-6582-5cc7-9226-f9367248776c","last_activity_at":"2026-11-05T01:00:00Z","name":"Northwind Traders","organisation_id":"da09c739-6e09-5587-a658-88f6b9eea0bd","phone":"+49 30 1015838","score":84,"tags":["waitlist"],"timezone":"Africa/Johannesburg"})).unwrap_or_default()],limit: 25,page: 1,total: 30,
    })
}
