// User-owned controller for handler 'create_deal_role'.

use crate::handlers::create_deal_role::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateDealRoleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "description": "Default role for the deal's first contact",
    //   "id": "c0fd9836-bb8e-5080-b832-50c4843b3e9f",
    //   "title": "Primary contact",
    //   "used_on": 38
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "description": "Default role for the deal's first contact",
  "id": "c0fd9836-bb8e-5080-b832-50c4843b3e9f",
  "title": "Primary contact",
  "used_on": 38
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        description: Some("Default role for the deal's first contact".to_string()),
        id: "c0fd9836-bb8e-5080-b832-50c4843b3e9f".to_string(),
        title: "Primary contact".to_string(),
        used_on: Some(38),
    })
}
