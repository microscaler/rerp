// User-owned controller for handler 'get_contact_accounts'.

use crate::handlers::get_contact_accounts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Organisation;

#[handler(GetContactAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
    //       "name": "Northwind Traders"
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
      "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
      "name": "Northwind Traders"
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
        items: vec![serde_json::from_value::<Organisation>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","id":"44f5daeb-faac-5a53-81fb-ac8b5f267d2c","name":"Northwind Traders"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
