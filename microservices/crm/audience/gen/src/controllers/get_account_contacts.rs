// User-owned controller for handler 'get_account_contacts'.

use crate::handlers::get_account_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Contact;

#[handler(GetAccountContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "email": "priya.natarajan@northwindtraders.example",
    //       "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
    //       "status": "SUBSCRIBED"
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
      "created_at": "2026-11-05T16:00:00Z",
      "email": "priya.natarajan@northwindtraders.example",
      "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
      "status": "SUBSCRIBED"
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
        items: vec![serde_json::from_value::<Contact>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","email":"priya.natarajan@northwindtraders.example","id":"44f5daeb-faac-5a53-81fb-ac8b5f267d2c","status":"SUBSCRIBED"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
