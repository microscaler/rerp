// User-owned controller for handler 'get_calendar_feed'.

use crate::handlers::get_calendar_feed::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetCalendarFeedController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "assignee_ids": [
    //     "alex"
    //   ],
    //   "created_at": "2026-11-03T16:00:00Z",
    //   "id": "da18346b-1f46-592f-ba4b-9d8012f04337",
    //   "include_alerts": true,
    //   "name": "Sales tasks",
    //   "pipeline_ids": [
    //     "d9a2022e-96ff-53a3-a94d-2b169424fd31"
    //   ],
    //   "scope": "DEALS",
    //   "token": "55e9e0da0f6b5bac785a4e086d4672cd",
    //   "url": "https://crm.acme.example/api/v1/crm/tasks/public/calendars/55e9e0da0f6b5bac785a4e086d4672cd.ics",
    //   "user_id": "alex"
    // }
    match serde_json::from_str::<Response>(
        r#"{
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
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        assignee_ids: Some(vec!["alex".to_string()]),created_at: "2026-11-03T16:00:00Z".to_string(),created_by: Some("example".to_string()),id: "da18346b-1f46-592f-ba4b-9d8012f04337".to_string(),include_alerts: Some(true),name: "Sales tasks".to_string(),pipeline_ids: Some(vec!["d9a2022e-96ff-53a3-a94d-2b169424fd31".to_string()]),scope: "DEALS".to_string(),token: Some("55e9e0da0f6b5bac785a4e086d4672cd".to_string()),url: Some("https://crm.acme.example/api/v1/crm/tasks/public/calendars/55e9e0da0f6b5bac785a4e086d4672cd.ics".to_string()),user_id: Some("alex".to_string()),
    })
}
