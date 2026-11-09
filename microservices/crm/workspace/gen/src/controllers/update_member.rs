// User-owned controller for handler 'update_member'.

use crate::handlers::update_member::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WorkspaceRole;

#[handler(UpdateMemberController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "added_at": "2026-11-05T16:00:00Z",
    //   "email": "priya.natarajan@northwindtraders.example",
    //   "last_active_at": "2026-11-05T16:00:00Z",
    //   "name": "Northwind Traders",
    //   "role": "owner",
    //   "user_id": "alex"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "added_at": "2026-11-05T16:00:00Z",
  "email": "priya.natarajan@northwindtraders.example",
  "last_active_at": "2026-11-05T16:00:00Z",
  "name": "Northwind Traders",
  "role": "owner",
  "user_id": "alex"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        added_at: Some("2026-11-05T16:00:00Z".to_string()),
        email: Some("priya.natarajan@northwindtraders.example".to_string()),
        last_active_at: Some("2026-11-05T16:00:00Z".to_string()),
        name: Some("Northwind Traders".to_string()),
        role: serde_json::from_value::<WorkspaceRole>(serde_json::Value::String(
            "owner".to_string(),
        ))
        .unwrap_or_default(),
        user_id: "alex".to_string(),
    })
}
