// User-owned controller for handler 'evaluate_segment'.

use crate::handlers::evaluate_segment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Contact;

#[handler(EvaluateSegmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "count": 412,
    //   "sample": [
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
    //     },
    //     {
    //       "create_date": "2026-11-05T16:00:00Z",
    //       "custom_fields": {
    //         "industry": "SaaS",
    //         "plan": "Free",
    //         "renewal_date": null,
    //         "seats": 14,
    //         "team_size": "1-10"
    //       },
    //       "email": "marcus.reid@adventureworks.example",
    //       "id": "93f1db66-d1af-580b-b546-6a13f1774d12",
    //       "last_activity_at": "2026-11-04T18:00:00Z",
    //       "name": "Northwind Traders",
    //       "organisation_id": "3cc2755d-bcec-5a0c-afa0-71d35560b07d",
    //       "phone": "+49 30 1023757",
    //       "score": 80,
    //       "tags": [
    //         "churned"
    //       ],
    //       "timezone": "America/New_York"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "count": 412,
  "sample": [
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
    },
    {
      "create_date": "2026-11-05T16:00:00Z",
      "custom_fields": {
        "industry": "SaaS",
        "plan": "Free",
        "renewal_date": null,
        "seats": 14,
        "team_size": "1-10"
      },
      "email": "marcus.reid@adventureworks.example",
      "id": "93f1db66-d1af-580b-b546-6a13f1774d12",
      "last_activity_at": "2026-11-04T18:00:00Z",
      "name": "Northwind Traders",
      "organisation_id": "3cc2755d-bcec-5a0c-afa0-71d35560b07d",
      "phone": "+49 30 1023757",
      "score": 80,
      "tags": [
        "churned"
      ],
      "timezone": "America/New_York"
    }
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        count: 412,sample: Some(vec![serde_json::from_value::<Contact>(serde_json::json!({"create_date":"2026-11-05T16:00:00Z","custom_fields":{"industry":"Retail","plan":"Team","renewal_date":"2026-12-14","seats":8,"team_size":"11-50"},"email":"tom.okafor@fabrikam.example","id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","last_activity_at":"2026-11-05T08:00:00Z","name":"Northwind Traders","organisation_id":"576a7630-a5b2-566c-ac00-caca59662cb0","phone":"+49 30 1007919","score":79,"tags":["trial"],"timezone":"Europe/London"})).unwrap_or_default(), serde_json::from_value::<Contact>(serde_json::json!({"create_date":"2026-11-05T16:00:00Z","custom_fields":{"industry":"Finance","plan":"Business","renewal_date":"2026-12-23","seats":11,"team_size":"51+"},"email":"lena.fischer@contoso.example","id":"f25ace60-6582-5cc7-9226-f9367248776c","last_activity_at":"2026-11-05T01:00:00Z","name":"Northwind Traders","organisation_id":"da09c739-6e09-5587-a658-88f6b9eea0bd","phone":"+49 30 1015838","score":84,"tags":["waitlist"],"timezone":"Africa/Johannesburg"})).unwrap_or_default(), serde_json::from_value::<Contact>(serde_json::json!({"create_date":"2026-11-05T16:00:00Z","custom_fields":{"industry":"SaaS","plan":"Free","renewal_date":null,"seats":14,"team_size":"1-10"},"email":"marcus.reid@adventureworks.example","id":"93f1db66-d1af-580b-b546-6a13f1774d12","last_activity_at":"2026-11-04T18:00:00Z","name":"Northwind Traders","organisation_id":"3cc2755d-bcec-5a0c-afa0-71d35560b07d","phone":"+49 30 1023757","score":80,"tags":["churned"],"timezone":"America/New_York"})).unwrap_or_default()]),
    })
}
