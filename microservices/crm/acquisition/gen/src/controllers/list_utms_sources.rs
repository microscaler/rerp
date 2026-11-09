// User-owned controller for handler 'list_utms_sources'.

use crate::handlers::list_utms_sources::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::UTMSource;

#[handler(ListUtmsSourcesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
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
      "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
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
        items: vec![serde_json::from_value::<UTMSource>(serde_json::json!({"id":"e67555dd-33f4-5fee-9da3-a019895219b1","name":"Northwind Traders"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
