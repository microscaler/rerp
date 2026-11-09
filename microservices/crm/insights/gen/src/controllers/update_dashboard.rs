// User-owned controller for handler 'update_dashboard'.

use crate::handlers::update_dashboard::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DashboardWidget;

#[handler(UpdateDashboardController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-10-26T16:00:00Z",
    //   "id": "d882d72a-b5af-5bb5-bd44-13bca206fd5b",
    //   "is_default": true,
    //   "name": "Alex's dashboard",
    //   "updated_at": "2026-11-05T15:59:00Z",
    //   "user_id": "alex",
    //   "widgets": [
    //     {
    //       "id": "91bbb6c0-47bc-562d-9ecb-6bd37b17b013",
    //       "kind": "NUMBERS",
    //       "position": 0,
    //       "query": {
    //         "metric": "email_kpis",
    //         "range": "LAST_30_DAYS"
    //       },
    //       "refreshed_at": "2026-11-05T15:59:00Z",
    //       "title": "Key email marketing metrics",
    //       "width": 2
    //     },
    //     {
    //       "id": "c55b2ef3-e9dc-5b6f-8a2a-fd6ae58cd32b",
    //       "kind": "BAR",
    //       "position": 1,
    //       "query": {
    //         "granularity": "MONTH",
    //         "metric": "total_contacts",
    //         "range": "LAST_12_MONTHS"
    //       },
    //       "refreshed_at": "2026-11-05T15:59:00Z",
    //       "title": "Contact growth over time",
    //       "width": 1
    //     },
    //     {
    //       "id": "dd78f406-2156-5e00-9f58-1c3cc539fc33",
    //       "kind": "TABLE",
    //       "position": 2,
    //       "query": {
    //         "dimension": "automation",
    //         "limit": 5,
    //         "metric": "automation_entries",
    //         "range": "ALL_TIME"
    //       },
    //       "refreshed_at": "2026-11-05T15:59:00Z",
    //       "title": "Most used automations by contacts entered",
    //       "width": 1
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created_at": "2026-10-26T16:00:00Z",
  "id": "d882d72a-b5af-5bb5-bd44-13bca206fd5b",
  "is_default": true,
  "name": "Alex's dashboard",
  "updated_at": "2026-11-05T15:59:00Z",
  "user_id": "alex",
  "widgets": [
    {
      "id": "91bbb6c0-47bc-562d-9ecb-6bd37b17b013",
      "kind": "NUMBERS",
      "position": 0,
      "query": {
        "metric": "email_kpis",
        "range": "LAST_30_DAYS"
      },
      "refreshed_at": "2026-11-05T15:59:00Z",
      "title": "Key email marketing metrics",
      "width": 2
    },
    {
      "id": "c55b2ef3-e9dc-5b6f-8a2a-fd6ae58cd32b",
      "kind": "BAR",
      "position": 1,
      "query": {
        "granularity": "MONTH",
        "metric": "total_contacts",
        "range": "LAST_12_MONTHS"
      },
      "refreshed_at": "2026-11-05T15:59:00Z",
      "title": "Contact growth over time",
      "width": 1
    },
    {
      "id": "dd78f406-2156-5e00-9f58-1c3cc539fc33",
      "kind": "TABLE",
      "position": 2,
      "query": {
        "dimension": "automation",
        "limit": 5,
        "metric": "automation_entries",
        "range": "ALL_TIME"
      },
      "refreshed_at": "2026-11-05T15:59:00Z",
      "title": "Most used automations by contacts entered",
      "width": 1
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
        created_at: Some("2026-10-26T16:00:00Z".to_string()),id: "d882d72a-b5af-5bb5-bd44-13bca206fd5b".to_string(),is_default: Some(true),name: "Alex's dashboard".to_string(),updated_at: Some("2026-11-05T15:59:00Z".to_string()),user_id: "alex".to_string(),widgets: vec![serde_json::from_value::<DashboardWidget>(serde_json::json!({"id":"91bbb6c0-47bc-562d-9ecb-6bd37b17b013","kind":"NUMBERS","position":0,"query":{"metric":"email_kpis","range":"LAST_30_DAYS"},"refreshed_at":"2026-11-05T15:59:00Z","title":"Key email marketing metrics","width":2})).unwrap_or_default(), serde_json::from_value::<DashboardWidget>(serde_json::json!({"id":"c55b2ef3-e9dc-5b6f-8a2a-fd6ae58cd32b","kind":"BAR","position":1,"query":{"granularity":"MONTH","metric":"total_contacts","range":"LAST_12_MONTHS"},"refreshed_at":"2026-11-05T15:59:00Z","title":"Contact growth over time","width":1})).unwrap_or_default(), serde_json::from_value::<DashboardWidget>(serde_json::json!({"id":"dd78f406-2156-5e00-9f58-1c3cc539fc33","kind":"TABLE","position":2,"query":{"dimension":"automation","limit":5,"metric":"automation_entries","range":"ALL_TIME"},"refreshed_at":"2026-11-05T15:59:00Z","title":"Most used automations by contacts entered","width":1})).unwrap_or_default()],
    })
}
