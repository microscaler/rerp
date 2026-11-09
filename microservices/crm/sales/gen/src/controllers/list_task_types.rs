// User-owned controller for handler 'list_task_types'.

use crate::handlers::list_task_types::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TaskType;

#[handler(ListTaskTypesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "active": true,
    //       "default_minutes": 15,
    //       "id": "4eec3526-5869-5e76-820f-ac2358c6b899",
    //       "open_count": 12,
    //       "outcomes": [
    //         "Connected",
    //         "No answer",
    //         "Left voicemail"
    //       ],
    //       "title": "Call"
    //     },
    //     {
    //       "active": true,
    //       "default_minutes": 0,
    //       "id": "099104fc-dcee-57bf-b906-d2b826beb51d",
    //       "open_count": 8,
    //       "outcomes": [
    //         "Sent",
    //         "Replied",
    //         "Bounced"
    //       ],
    //       "title": "Email"
    //     },
    //     {
    //       "active": true,
    //       "default_minutes": 45,
    //       "id": "645bfe17-69c3-558e-82cd-9594c3609fa2",
    //       "open_count": 5,
    //       "outcomes": [
    //         "Held",
    //         "No-show",
    //         "Rescheduled"
    //       ],
    //       "title": "Meeting"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 4
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "active": true,
      "default_minutes": 15,
      "id": "4eec3526-5869-5e76-820f-ac2358c6b899",
      "open_count": 12,
      "outcomes": [
        "Connected",
        "No answer",
        "Left voicemail"
      ],
      "title": "Call"
    },
    {
      "active": true,
      "default_minutes": 0,
      "id": "099104fc-dcee-57bf-b906-d2b826beb51d",
      "open_count": 8,
      "outcomes": [
        "Sent",
        "Replied",
        "Bounced"
      ],
      "title": "Email"
    },
    {
      "active": true,
      "default_minutes": 45,
      "id": "645bfe17-69c3-558e-82cd-9594c3609fa2",
      "open_count": 5,
      "outcomes": [
        "Held",
        "No-show",
        "Rescheduled"
      ],
      "title": "Meeting"
    }
  ],
  "limit": 25,
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
        items: vec![serde_json::from_value::<TaskType>(serde_json::json!({"active":true,"default_minutes":15,"id":"4eec3526-5869-5e76-820f-ac2358c6b899","open_count":12,"outcomes":["Connected","No answer","Left voicemail"],"title":"Call"})).unwrap_or_default(), serde_json::from_value::<TaskType>(serde_json::json!({"active":true,"default_minutes":0,"id":"099104fc-dcee-57bf-b906-d2b826beb51d","open_count":8,"outcomes":["Sent","Replied","Bounced"],"title":"Email"})).unwrap_or_default(), serde_json::from_value::<TaskType>(serde_json::json!({"active":true,"default_minutes":45,"id":"645bfe17-69c3-558e-82cd-9594c3609fa2","open_count":5,"outcomes":["Held","No-show","Rescheduled"],"title":"Meeting"})).unwrap_or_default()],limit: 25,page: 1,total: 4,
    })
}
