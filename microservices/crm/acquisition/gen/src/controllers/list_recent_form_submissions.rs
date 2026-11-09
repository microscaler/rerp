// User-owned controller for handler 'list_recent_form_submissions'.

use crate::handlers::list_recent_form_submissions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FormSubmission;

#[handler(ListRecentFormSubmissionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_id": "5de76105-f8c3-5357-af13-c5ce28753ed0",
    //       "form_id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
    //       "id": "33862d04-0db3-5f10-8b9e-da432aa545ec",
    //       "submitted_at": "2026-11-05T16:00:00Z",
    //       "values": {
    //         "key": "Value"
    //       }
    //     },
    //     {
    //       "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "form_id": "d614ffd8-570e-5b8f-b84c-75324e12322e",
    //       "id": "77a87e6a-0d20-5e66-ac8b-b889d993c003",
    //       "submitted_at": "2026-11-05T16:00:00Z",
    //       "values": {
    //         "key": "Value"
    //       }
    //     },
    //     {
    //       "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "form_id": "997e0128-1f73-5fb4-b47e-b72197474a68",
    //       "id": "b14461b9-7786-5357-9035-86f330642d09",
    //       "submitted_at": "2026-11-05T16:00:00Z",
    //       "values": {
    //         "key": "Value"
    //       }
    //     }
    //   ],
    //   "limit": 6,
    //   "page": 1,
    //   "total": 4
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "contact_id": "5de76105-f8c3-5357-af13-c5ce28753ed0",
      "form_id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
      "id": "33862d04-0db3-5f10-8b9e-da432aa545ec",
      "submitted_at": "2026-11-05T16:00:00Z",
      "values": {
        "key": "Value"
      }
    },
    {
      "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "form_id": "d614ffd8-570e-5b8f-b84c-75324e12322e",
      "id": "77a87e6a-0d20-5e66-ac8b-b889d993c003",
      "submitted_at": "2026-11-05T16:00:00Z",
      "values": {
        "key": "Value"
      }
    },
    {
      "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "form_id": "997e0128-1f73-5fb4-b47e-b72197474a68",
      "id": "b14461b9-7786-5357-9035-86f330642d09",
      "submitted_at": "2026-11-05T16:00:00Z",
      "values": {
        "key": "Value"
      }
    }
  ],
  "limit": 6,
  "page": 1,
  "total": 4
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<FormSubmission>(serde_json::json!({"contact_id":"5de76105-f8c3-5357-af13-c5ce28753ed0","form_id":"454cdf11-2c7b-505e-8864-df16d18ca8cc","id":"33862d04-0db3-5f10-8b9e-da432aa545ec","submitted_at":"2026-11-05T16:00:00Z","values":{"key":"Value"}})).unwrap_or_default(), serde_json::from_value::<FormSubmission>(serde_json::json!({"contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","form_id":"d614ffd8-570e-5b8f-b84c-75324e12322e","id":"77a87e6a-0d20-5e66-ac8b-b889d993c003","submitted_at":"2026-11-05T16:00:00Z","values":{"key":"Value"}})).unwrap_or_default(), serde_json::from_value::<FormSubmission>(serde_json::json!({"contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","form_id":"997e0128-1f73-5fb4-b47e-b72197474a68","id":"b14461b9-7786-5357-9035-86f330642d09","submitted_at":"2026-11-05T16:00:00Z","values":{"key":"Value"}})).unwrap_or_default()],limit: 6,page: 1,total: 4,
    })
}
