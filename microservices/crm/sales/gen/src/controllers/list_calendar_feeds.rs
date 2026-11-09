// User-owned controller for handler 'list_calendar_feeds'.

use crate::handlers::list_calendar_feeds::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CalendarFeed;

#[handler(ListCalendarFeedsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "assignee_ids": [
    //         "alex"
    //       ],
    //       "created_at": "2026-11-03T16:00:00Z",
    //       "id": "da18346b-1f46-592f-ba4b-9d8012f04337",
    //       "include_alerts": true,
    //       "name": "Sales tasks",
    //       "pipeline_ids": [
    //         "d9a2022e-96ff-53a3-a94d-2b169424fd31"
    //       ],
    //       "scope": "DEALS",
    //       "token": "55e9e0da0f6b5bac785a4e086d4672cd",
    //       "url": "https://crm.acme.example/api/v1/crm/tasks/public/calendars/55e9e0da0f6b5bac785a4e086d4672cd.ics",
    //       "user_id": "alex"
    //     },
    //     {
    //       "assignee_ids": [],
    //       "created_at": "2026-09-26T16:00:00Z",
    //       "id": "e094760a-c9e6-5722-a46b-a81f93262000",
    //       "include_alerts": false,
    //       "name": "Everything",
    //       "pipeline_ids": [],
    //       "scope": "ALL",
    //       "token": "a91c2f0e7b8d4c6a9e1f3b5d7c9e04f1",
    //       "url": "https://crm.acme.example/api/v1/crm/tasks/public/calendars/a91c2f0e7b8d4c6a9e1f3b5d7c9e04f1.ics",
    //       "user_id": "alex"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "assignee_ids": [
        "alex"
      ],
      "created_at": "2026-11-03T16:00:00Z",
      "id": "da18346b-1f46-592f-ba4b-9d8012f04337",
      "include_alerts": true,
      "name": "Sales tasks",
      "pipeline_ids": [
        "d9a2022e-96ff-53a3-a94d-2b169424fd31"
      ],
      "scope": "DEALS",
      "token": "55e9e0da0f6b5bac785a4e086d4672cd",
      "url": "https://crm.acme.example/api/v1/crm/tasks/public/calendars/55e9e0da0f6b5bac785a4e086d4672cd.ics",
      "user_id": "alex"
    },
    {
      "assignee_ids": [],
      "created_at": "2026-09-26T16:00:00Z",
      "id": "e094760a-c9e6-5722-a46b-a81f93262000",
      "include_alerts": false,
      "name": "Everything",
      "pipeline_ids": [],
      "scope": "ALL",
      "token": "a91c2f0e7b8d4c6a9e1f3b5d7c9e04f1",
      "url": "https://crm.acme.example/api/v1/crm/tasks/public/calendars/a91c2f0e7b8d4c6a9e1f3b5d7c9e04f1.ics",
      "user_id": "alex"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<CalendarFeed>(serde_json::json!({"assignee_ids":["alex"],"created_at":"2026-11-03T16:00:00Z","id":"da18346b-1f46-592f-ba4b-9d8012f04337","include_alerts":true,"name":"Sales tasks","pipeline_ids":["d9a2022e-96ff-53a3-a94d-2b169424fd31"],"scope":"DEALS","token":"55e9e0da0f6b5bac785a4e086d4672cd","url":"https://crm.acme.example/api/v1/crm/tasks/public/calendars/55e9e0da0f6b5bac785a4e086d4672cd.ics","user_id":"alex"})).unwrap_or_default(), serde_json::from_value::<CalendarFeed>(serde_json::json!({"assignee_ids":[],"created_at":"2026-09-26T16:00:00Z","id":"e094760a-c9e6-5722-a46b-a81f93262000","include_alerts":false,"name":"Everything","pipeline_ids":[],"scope":"ALL","token":"a91c2f0e7b8d4c6a9e1f3b5d7c9e04f1","url":"https://crm.acme.example/api/v1/crm/tasks/public/calendars/a91c2f0e7b8d4c6a9e1f3b5d7c9e04f1.ics","user_id":"alex"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
