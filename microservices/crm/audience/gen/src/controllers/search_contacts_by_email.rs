// User-owned controller for handler 'search_contacts_by_email'.

use crate::handlers::search_contacts_by_email::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Contact;

#[handler(SearchContactsByEmailController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "create_date": "2026-11-05T16:00:00Z",
    //       "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
    //       "name": "Northwind Traders"
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
      "create_date": "2026-11-05T16:00:00Z",
      "id": "44f5daeb-faac-5a53-81fb-ac8b5f267d2c",
      "name": "Northwind Traders"
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
        items: vec![serde_json::from_value::<Contact>(serde_json::json!({"create_date":"2026-11-05T16:00:00Z","id":"44f5daeb-faac-5a53-81fb-ac8b5f267d2c","name":"Northwind Traders"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
