// User-owned controller for handler 'list_pipelines'.

use crate::handlers::list_pipelines::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Pipeline;

#[handler(ListPipelinesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "auto_assign": "ROUND_ROBIN",
    //       "closed_deals": 7,
    //       "created_at": "2026-03-30T16:00:00Z",
    //       "currency": "EUR",
    //       "fiscal_year_start": "JAN",
    //       "id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //       "name": "Sales",
    //       "open_deals": 38,
    //       "open_value": "412300.00",
    //       "visibility": "ALL"
    //     },
    //     {
    //       "auto_assign": "OFF",
    //       "closed_deals": 2,
    //       "created_at": "2026-03-30T16:00:00Z",
    //       "currency": "EUR",
    //       "fiscal_year_start": "JAN",
    //       "id": "7bb3c22d-2ff5-5dd8-8252-93551ce02c7c",
    //       "name": "Partners",
    //       "open_deals": 6,
    //       "open_value": "48000.00",
    //       "visibility": "ALL"
    //     },
    //     {
    //       "auto_assign": "OFF",
    //       "closed_deals": 2,
    //       "created_at": "2026-03-30T16:00:00Z",
    //       "currency": "EUR",
    //       "fiscal_year_start": "JAN",
    //       "id": "0b83cac7-7b15-5d24-b8bd-bb5960b6bec6",
    //       "name": "Renewals",
    //       "open_deals": 11,
    //       "open_value": "96500.00",
    //       "visibility": "ALL"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "auto_assign": "ROUND_ROBIN",
      "closed_deals": 7,
      "created_at": "2026-03-30T16:00:00Z",
      "currency": "EUR",
      "fiscal_year_start": "JAN",
      "id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
      "name": "Sales",
      "open_deals": 38,
      "open_value": "412300.00",
      "visibility": "ALL"
    },
    {
      "auto_assign": "OFF",
      "closed_deals": 2,
      "created_at": "2026-03-30T16:00:00Z",
      "currency": "EUR",
      "fiscal_year_start": "JAN",
      "id": "7bb3c22d-2ff5-5dd8-8252-93551ce02c7c",
      "name": "Partners",
      "open_deals": 6,
      "open_value": "48000.00",
      "visibility": "ALL"
    },
    {
      "auto_assign": "OFF",
      "closed_deals": 2,
      "created_at": "2026-03-30T16:00:00Z",
      "currency": "EUR",
      "fiscal_year_start": "JAN",
      "id": "0b83cac7-7b15-5d24-b8bd-bb5960b6bec6",
      "name": "Renewals",
      "open_deals": 11,
      "open_value": "96500.00",
      "visibility": "ALL"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 3
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Pipeline>(serde_json::json!({"auto_assign":"ROUND_ROBIN","closed_deals":7,"created_at":"2026-03-30T16:00:00Z","currency":"EUR","fiscal_year_start":"JAN","id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","name":"Sales","open_deals":38,"open_value":"412300.00","visibility":"ALL"})).unwrap_or_default(), serde_json::from_value::<Pipeline>(serde_json::json!({"auto_assign":"OFF","closed_deals":2,"created_at":"2026-03-30T16:00:00Z","currency":"EUR","fiscal_year_start":"JAN","id":"7bb3c22d-2ff5-5dd8-8252-93551ce02c7c","name":"Partners","open_deals":6,"open_value":"48000.00","visibility":"ALL"})).unwrap_or_default(), serde_json::from_value::<Pipeline>(serde_json::json!({"auto_assign":"OFF","closed_deals":2,"created_at":"2026-03-30T16:00:00Z","currency":"EUR","fiscal_year_start":"JAN","id":"0b83cac7-7b15-5d24-b8bd-bb5960b6bec6","name":"Renewals","open_deals":11,"open_value":"96500.00","visibility":"ALL"})).unwrap_or_default()],limit: 25,page: 1,total: 3,
    })
}
