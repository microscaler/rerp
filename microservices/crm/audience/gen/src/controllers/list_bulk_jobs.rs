// User-owned controller for handler 'list_bulk_jobs'.

use crate::handlers::list_bulk_jobs::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BulkJob;

#[handler(ListBulkJobsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "action": "ADD_TAG",
    //       "by": "Riya Kapoor",
    //       "contacts": 418,
    //       "created_at": "2026-10-23T16:00:00Z",
    //       "id": "c16c46aa-99a8-54ba-a51c-952111d137c1",
    //       "label": "Add tag webinar-oct",
    //       "status": "DONE",
    //       "undoable": true,
    //       "value": "webinar-oct"
    //     },
    //     {
    //       "action": "SET_FIELD",
    //       "by": "Import · partners.csv",
    //       "contacts": 96,
    //       "created_at": "2026-09-14T16:00:00Z",
    //       "id": "8ce6b7ec-d07d-5cb6-adbd-b95e66a7a596",
    //       "label": "Set field plan = Team",
    //       "status": "DONE",
    //       "undoable": false,
    //       "value": "plan = Team"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "action": "ADD_TAG",
      "by": "Riya Kapoor",
      "contacts": 418,
      "created_at": "2026-10-23T16:00:00Z",
      "id": "c16c46aa-99a8-54ba-a51c-952111d137c1",
      "label": "Add tag webinar-oct",
      "status": "DONE",
      "undoable": true,
      "value": "webinar-oct"
    },
    {
      "action": "SET_FIELD",
      "by": "Import · partners.csv",
      "contacts": 96,
      "created_at": "2026-09-14T16:00:00Z",
      "id": "8ce6b7ec-d07d-5cb6-adbd-b95e66a7a596",
      "label": "Set field plan = Team",
      "status": "DONE",
      "undoable": false,
      "value": "plan = Team"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<BulkJob>(serde_json::json!({"action":"ADD_TAG","by":"Riya Kapoor","contacts":418,"created_at":"2026-10-23T16:00:00Z","id":"c16c46aa-99a8-54ba-a51c-952111d137c1","label":"Add tag webinar-oct","status":"DONE","undoable":true,"value":"webinar-oct"})).unwrap_or_default(), serde_json::from_value::<BulkJob>(serde_json::json!({"action":"SET_FIELD","by":"Import · partners.csv","contacts":96,"created_at":"2026-09-14T16:00:00Z","id":"8ce6b7ec-d07d-5cb6-adbd-b95e66a7a596","label":"Set field plan = Team","status":"DONE","undoable":false,"value":"plan = Team"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
