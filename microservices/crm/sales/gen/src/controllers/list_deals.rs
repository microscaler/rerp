// User-owned controller for handler 'list_deals'.

use crate::handlers::list_deals::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Deal;

#[handler(ListDealsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contacts": [
    //         {
    //           "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //           "name": "Priya Natarajan",
    //           "role": "Primary contact"
    //         }
    //       ],
    //       "created_at": "2026-10-13T16:00:00Z",
    //       "currency": "EUR",
    //       "custom_fields": {
    //         "seats": 10,
    //         "term_months": "12"
    //       },
    //       "days_in_stage": 3,
    //       "expected_close_date": "2026-11-25",
    //       "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "last_activity_at": "2026-11-02T16:00:00Z",
    //       "next_task_at": "2026-11-06T16:00:00Z",
    //       "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //       "organisation_name": "Northwind Traders",
    //       "owner_id": "alex",
    //       "owner_name": "Alex Morgan",
    //       "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //       "probability": 50,
    //       "source": "customer expansion",
    //       "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //       "stage_name": "Proposal",
    //       "status": "OPEN",
    //       "title": "Northwind · 48 → 120 seats",
    //       "updated_at": "2026-11-02T16:00:00Z",
    //       "value": "38400.00",
    //       "weighted_value": "19200.00"
    //     },
    //     {
    //       "contacts": [
    //         {
    //           "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //           "name": "Tom Okafor",
    //           "role": "Primary contact"
    //         }
    //       ],
    //       "created_at": "2026-10-14T16:00:00Z",
    //       "currency": "EUR",
    //       "custom_fields": {
    //         "seats": 14,
    //         "term_months": "12"
    //       },
    //       "days_in_stage": 2,
    //       "expected_close_date": "2026-12-01",
    //       "id": "48d65ced-2142-5626-af8e-c006f3c081b0",
    //       "last_activity_at": "2026-11-03T16:00:00Z",
    //       "next_task_at": "2026-11-07T16:00:00Z",
    //       "organisation_id": "576a7630-a5b2-566c-ac00-caca59662cb0",
    //       "organisation_name": "Fabrikam Inc",
    //       "owner_id": "sam",
    //       "owner_name": "Sam Lee",
    //       "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //       "probability": 10,
    //       "source": "form: free trial",
    //       "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
    //       "stage_name": "New",
    //       "status": "OPEN",
    //       "title": "Fabrikam · trial → team",
    //       "updated_at": "2026-11-03T16:00:00Z",
    //       "value": "4800.00",
    //       "weighted_value": "480.00"
    //     },
    //     {
    //       "contacts": [
    //         {
    //           "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //           "name": "Lena Fischer",
    //           "role": "Primary contact"
    //         }
    //       ],
    //       "created_at": "2026-10-08T16:00:00Z",
    //       "currency": "EUR",
    //       "custom_fields": {
    //         "seats": 18,
    //         "term_months": "12"
    //       },
    //       "days_in_stage": 8,
    //       "expected_close_date": "2026-12-07",
    //       "id": "9c1e9e9d-0f28-58fc-bef5-2374e4abf12c",
    //       "last_activity_at": "2026-10-28T16:00:00Z",
    //       "next_task_at": "2026-11-08T16:00:00Z",
    //       "organisation_id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
    //       "organisation_name": "Contoso Ltd",
    //       "owner_id": "riya",
    //       "owner_name": "Riya Kapoor",
    //       "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //       "probability": 50,
    //       "source": "form: contact sales",
    //       "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //       "stage_name": "Proposal",
    //       "status": "OPEN",
    //       "title": "Contoso · Business plan",
    //       "updated_at": "2026-10-28T16:00:00Z",
    //       "value": "24000.00",
    //       "weighted_value": "12000.00"
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
    },
    {
      "contacts": [
        {
          "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
          "name": "Tom Okafor",
          "role": "Primary contact"
        }
      ],
      "created_at": "2026-10-14T16:00:00Z",
      "currency": "EUR",
      "custom_fields": {
        "seats": 14,
        "term_months": "12"
      },
      "days_in_stage": 2,
      "expected_close_date": "2026-12-01",
      "id": "48d65ced-2142-5626-af8e-c006f3c081b0",
      "last_activity_at": "2026-11-03T16:00:00Z",
      "next_task_at": "2026-11-07T16:00:00Z",
      "organisation_id": "576a7630-a5b2-566c-ac00-caca59662cb0",
      "organisation_name": "Fabrikam Inc",
      "owner_id": "sam",
      "owner_name": "Sam Lee",
      "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
      "probability": 10,
      "source": "form: free trial",
      "stage_id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
      "stage_name": "New",
      "status": "OPEN",
      "title": "Fabrikam · trial → team",
      "updated_at": "2026-11-03T16:00:00Z",
      "value": "4800.00",
      "weighted_value": "480.00"
    },
    {
      "contacts": [
        {
          "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
          "name": "Lena Fischer",
          "role": "Primary contact"
        }
      ],
      "created_at": "2026-10-08T16:00:00Z",
      "currency": "EUR",
      "custom_fields": {
        "seats": 18,
        "term_months": "12"
      },
      "days_in_stage": 8,
      "expected_close_date": "2026-12-07",
      "id": "9c1e9e9d-0f28-58fc-bef5-2374e4abf12c",
      "last_activity_at": "2026-10-28T16:00:00Z",
      "next_task_at": "2026-11-08T16:00:00Z",
      "organisation_id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
      "organisation_name": "Contoso Ltd",
      "owner_id": "riya",
      "owner_name": "Riya Kapoor",
      "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
      "probability": 50,
      "source": "form: contact sales",
      "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
      "stage_name": "Proposal",
      "status": "OPEN",
      "title": "Contoso · Business plan",
      "updated_at": "2026-10-28T16:00:00Z",
      "value": "24000.00",
      "weighted_value": "12000.00"
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
        items: vec![serde_json::from_value::<Deal>(serde_json::json!({"contacts":[{"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","name":"Priya Natarajan","role":"Primary contact"}],"created_at":"2026-10-13T16:00:00Z","currency":"EUR","custom_fields":{"seats":10,"term_months":"12"},"days_in_stage":3,"expected_close_date":"2026-11-25","id":"63a6661f-5860-50ed-8697-f28d29a3917f","last_activity_at":"2026-11-02T16:00:00Z","next_task_at":"2026-11-06T16:00:00Z","organisation_id":"d7a2e2b8-61b1-5687-8aee-0ead8943ba3b","organisation_name":"Northwind Traders","owner_id":"alex","owner_name":"Alex Morgan","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":50,"source":"customer expansion","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","status":"OPEN","title":"Northwind · 48 → 120 seats","updated_at":"2026-11-02T16:00:00Z","value":"38400.00","weighted_value":"19200.00"})).unwrap_or_default(), serde_json::from_value::<Deal>(serde_json::json!({"contacts":[{"contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","name":"Tom Okafor","role":"Primary contact"}],"created_at":"2026-10-14T16:00:00Z","currency":"EUR","custom_fields":{"seats":14,"term_months":"12"},"days_in_stage":2,"expected_close_date":"2026-12-01","id":"48d65ced-2142-5626-af8e-c006f3c081b0","last_activity_at":"2026-11-03T16:00:00Z","next_task_at":"2026-11-07T16:00:00Z","organisation_id":"576a7630-a5b2-566c-ac00-caca59662cb0","organisation_name":"Fabrikam Inc","owner_id":"sam","owner_name":"Sam Lee","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":10,"source":"form: free trial","stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","stage_name":"New","status":"OPEN","title":"Fabrikam · trial → team","updated_at":"2026-11-03T16:00:00Z","value":"4800.00","weighted_value":"480.00"})).unwrap_or_default(), serde_json::from_value::<Deal>(serde_json::json!({"contacts":[{"contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","name":"Lena Fischer","role":"Primary contact"}],"created_at":"2026-10-08T16:00:00Z","currency":"EUR","custom_fields":{"seats":18,"term_months":"12"},"days_in_stage":8,"expected_close_date":"2026-12-07","id":"9c1e9e9d-0f28-58fc-bef5-2374e4abf12c","last_activity_at":"2026-10-28T16:00:00Z","next_task_at":"2026-11-08T16:00:00Z","organisation_id":"da09c739-6e09-5587-a658-88f6b9eea0bd","organisation_name":"Contoso Ltd","owner_id":"riya","owner_name":"Riya Kapoor","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":50,"source":"form: contact sales","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","status":"OPEN","title":"Contoso · Business plan","updated_at":"2026-10-28T16:00:00Z","value":"24000.00","weighted_value":"12000.00"})).unwrap_or_default()],limit: 25,page: 1,total: 12,
    })
}
