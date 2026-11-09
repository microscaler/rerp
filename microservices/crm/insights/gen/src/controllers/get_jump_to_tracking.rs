// User-owned controller for handler 'get_jump_to_tracking'.

use crate::handlers::get_jump_to_tracking::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::JumpToRow;

#[handler(GetJumpToTrackingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contacts_jumped": 377,
    //       "contacts_passed": 1025,
    //       "goal_name": "Activated the product",
    //       "jump_rate": 0.23,
    //       "step_id": "047a18d8-3dbe-5b48-8733-4d69800e8319",
    //       "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //       "workflow_name": "Welcome series (7 days)"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "contacts_jumped": 377,
      "contacts_passed": 1025,
      "goal_name": "Activated the product",
      "jump_rate": 0.23,
      "step_id": "047a18d8-3dbe-5b48-8733-4d69800e8319",
      "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
      "workflow_name": "Welcome series (7 days)"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 1
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<JumpToRow>(serde_json::json!({"contacts_jumped":377,"contacts_passed":1025,"goal_name":"Activated the product","jump_rate":0.23,"step_id":"047a18d8-3dbe-5b48-8733-4d69800e8319","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"})).unwrap_or_default()],limit: 20,page: 1,total: 1,
    })
}
