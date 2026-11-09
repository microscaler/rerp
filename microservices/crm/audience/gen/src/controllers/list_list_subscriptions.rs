// User-owned controller for handler 'list_list_subscriptions'.

use crate::handlers::list_list_subscriptions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Subscription;

#[handler(ListListSubscriptionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_id": "07a7b393-a2b8-5f9d-86c2-37bc92d15398",
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
    //       "list_id": "217107c2-57c1-5880-b574-99c272405f82",
    //       "status": "PENDING"
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
      "contact_id": "07a7b393-a2b8-5f9d-86c2-37bc92d15398",
      "created_at": "2026-11-05T16:00:00Z",
      "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
      "list_id": "217107c2-57c1-5880-b574-99c272405f82",
      "status": "PENDING"
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
        items: vec![serde_json::from_value::<Subscription>(serde_json::json!({"contact_id":"07a7b393-a2b8-5f9d-86c2-37bc92d15398","created_at":"2026-11-05T16:00:00Z","id":"44f5daeb-faac-5a53-81fb-ac8b5f267d2c","list_id":"217107c2-57c1-5880-b574-99c272405f82","status":"PENDING"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
