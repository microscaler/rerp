// User-owned controller for handler 'list_leads'.

use crate::handlers::list_leads::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Lead;

#[handler(ListLeadsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "active": true,
    //       "contact_name": "Priya Natarajan",
    //       "create_date": "2026-10-13T16:00:00Z",
    //       "date_last_stage_update": "2026-11-02T16:00:00Z",
    //       "description": "Source: customer expansion",
    //       "email_from": "priya.natarajan@northwindtraders.example",
    //       "expected_revenue": "38400.00",
    //       "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "name": "Northwind · 48 → 120 seats",
    //       "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //       "organisation_name": "Northwind Traders",
    //       "phone": "+49 30 1000000",
    //       "priority": "NORMAL",
    //       "probability": 50,
    //       "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //       "stage_name": "Proposal",
    //       "stage_probability": 50,
    //       "tag_ids": [],
    //       "team_id": "00000000-0000-0000-0000-000000000401",
    //       "type": "LEAD",
    //       "won_status": "PENDING",
    //       "write_date": "2026-11-02T16:00:00Z"
    //     },
    //     {
    //       "active": true,
    //       "contact_name": "Tom Okafor",
    //       "create_date": "2026-10-14T16:00:00Z",
    //       "date_last_stage_update": "2026-11-03T16:00:00Z",
    //       "description": "Source: form: free trial",
    //       "email_from": "tom.okafor@fabrikam.example",
    //       "expected_revenue": "4800.00",
    //       "id": "48d65ced-2142-5626-af8e-c006f3c081b0",
    //       "name": "Fabrikam · trial → team",
    //       "organisation_id": "576a7630-a5b2-566c-ac00-caca59662cb0",
    //       "organisation_name": "Fabrikam Inc",
    //       "phone": "+49 30 1007919",
    //       "priority": "HIGH",
    //       "probability": 10,
    //       "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
    //       "stage_name": "New",
    //       "stage_probability": 10,
    //       "tag_ids": [],
    //       "team_id": "00000000-0000-0000-0000-000000000401",
    //       "type": "LEAD",
    //       "won_status": "PENDING",
    //       "write_date": "2026-11-03T16:00:00Z"
    //     },
    //     {
    //       "active": true,
    //       "contact_name": "Lena Fischer",
    //       "create_date": "2026-10-08T16:00:00Z",
    //       "date_last_stage_update": "2026-10-28T16:00:00Z",
    //       "description": "Source: form: contact sales",
    //       "email_from": "lena.fischer@contoso.example",
    //       "expected_revenue": "24000.00",
    //       "id": "9c1e9e9d-0f28-58fc-bef5-2374e4abf12c",
    //       "name": "Contoso · Business plan",
    //       "organisation_id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
    //       "organisation_name": "Contoso Ltd",
    //       "phone": "+49 30 1015838",
    //       "priority": "LOW",
    //       "probability": 50,
    //       "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //       "stage_name": "Proposal",
    //       "stage_probability": 50,
    //       "tag_ids": [],
    //       "team_id": "00000000-0000-0000-0000-000000000401",
    //       "type": "LEAD",
    //       "won_status": "PENDING",
    //       "write_date": "2026-10-28T16:00:00Z"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 12
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "active": true,
      "contact_name": "Priya Natarajan",
      "create_date": "2026-10-13T16:00:00Z",
      "date_last_stage_update": "2026-11-02T16:00:00Z",
      "description": "Source: customer expansion",
      "email_from": "priya.natarajan@northwindtraders.example",
      "expected_revenue": "38400.00",
      "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
      "name": "Northwind · 48 → 120 seats",
      "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
      "organisation_name": "Northwind Traders",
      "phone": "+49 30 1000000",
      "priority": "NORMAL",
      "probability": 50,
      "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
      "stage_name": "Proposal",
      "stage_probability": 50,
      "tag_ids": [],
      "team_id": "00000000-0000-0000-0000-000000000401",
      "type": "LEAD",
      "won_status": "PENDING",
      "write_date": "2026-11-02T16:00:00Z"
    },
    {
      "active": true,
      "contact_name": "Tom Okafor",
      "create_date": "2026-10-14T16:00:00Z",
      "date_last_stage_update": "2026-11-03T16:00:00Z",
      "description": "Source: form: free trial",
      "email_from": "tom.okafor@fabrikam.example",
      "expected_revenue": "4800.00",
      "id": "48d65ced-2142-5626-af8e-c006f3c081b0",
      "name": "Fabrikam · trial → team",
      "organisation_id": "576a7630-a5b2-566c-ac00-caca59662cb0",
      "organisation_name": "Fabrikam Inc",
      "phone": "+49 30 1007919",
      "priority": "HIGH",
      "probability": 10,
      "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
      "stage_name": "New",
      "stage_probability": 10,
      "tag_ids": [],
      "team_id": "00000000-0000-0000-0000-000000000401",
      "type": "LEAD",
      "won_status": "PENDING",
      "write_date": "2026-11-03T16:00:00Z"
    },
    {
      "active": true,
      "contact_name": "Lena Fischer",
      "create_date": "2026-10-08T16:00:00Z",
      "date_last_stage_update": "2026-10-28T16:00:00Z",
      "description": "Source: form: contact sales",
      "email_from": "lena.fischer@contoso.example",
      "expected_revenue": "24000.00",
      "id": "9c1e9e9d-0f28-58fc-bef5-2374e4abf12c",
      "name": "Contoso · Business plan",
      "organisation_id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
      "organisation_name": "Contoso Ltd",
      "phone": "+49 30 1015838",
      "priority": "LOW",
      "probability": 50,
      "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
      "stage_name": "Proposal",
      "stage_probability": 50,
      "tag_ids": [],
      "team_id": "00000000-0000-0000-0000-000000000401",
      "type": "LEAD",
      "won_status": "PENDING",
      "write_date": "2026-10-28T16:00:00Z"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 12
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Lead>(serde_json::json!({"active":true,"contact_name":"Priya Natarajan","create_date":"2026-10-13T16:00:00Z","date_last_stage_update":"2026-11-02T16:00:00Z","description":"Source: customer expansion","email_from":"priya.natarajan@northwindtraders.example","expected_revenue":"38400.00","id":"63a6661f-5860-50ed-8697-f28d29a3917f","name":"Northwind · 48 → 120 seats","organisation_id":"d7a2e2b8-61b1-5687-8aee-0ead8943ba3b","organisation_name":"Northwind Traders","phone":"+49 30 1000000","priority":"NORMAL","probability":50,"stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","stage_probability":50,"tag_ids":[],"team_id":"00000000-0000-0000-0000-000000000401","type":"LEAD","won_status":"PENDING","write_date":"2026-11-02T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Lead>(serde_json::json!({"active":true,"contact_name":"Tom Okafor","create_date":"2026-10-14T16:00:00Z","date_last_stage_update":"2026-11-03T16:00:00Z","description":"Source: form: free trial","email_from":"tom.okafor@fabrikam.example","expected_revenue":"4800.00","id":"48d65ced-2142-5626-af8e-c006f3c081b0","name":"Fabrikam · trial → team","organisation_id":"576a7630-a5b2-566c-ac00-caca59662cb0","organisation_name":"Fabrikam Inc","phone":"+49 30 1007919","priority":"HIGH","probability":10,"stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","stage_name":"New","stage_probability":10,"tag_ids":[],"team_id":"00000000-0000-0000-0000-000000000401","type":"LEAD","won_status":"PENDING","write_date":"2026-11-03T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Lead>(serde_json::json!({"active":true,"contact_name":"Lena Fischer","create_date":"2026-10-08T16:00:00Z","date_last_stage_update":"2026-10-28T16:00:00Z","description":"Source: form: contact sales","email_from":"lena.fischer@contoso.example","expected_revenue":"24000.00","id":"9c1e9e9d-0f28-58fc-bef5-2374e4abf12c","name":"Contoso · Business plan","organisation_id":"da09c739-6e09-5587-a658-88f6b9eea0bd","organisation_name":"Contoso Ltd","phone":"+49 30 1015838","priority":"LOW","probability":50,"stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","stage_probability":50,"tag_ids":[],"team_id":"00000000-0000-0000-0000-000000000401","type":"LEAD","won_status":"PENDING","write_date":"2026-10-28T16:00:00Z"})).unwrap_or_default()],limit: 25,page: 1,total: 12,
    })
}
