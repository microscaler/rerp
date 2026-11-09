// User-owned controller for handler 'create_utms_source'.

use crate::handlers::create_utms_source::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateUtmsSourceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "active": true,
    //   "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //   "medium_id": "f4ffaf26-ce12-5be0-9f99-e249743679ac",
    //   "name": "Northwind Traders"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "active": true,
  "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
  "medium_id": "f4ffaf26-ce12-5be0-9f99-e249743679ac",
  "name": "Northwind Traders"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        active: Some(true),
        id: "e67555dd-33f4-5fee-9da3-a019895219b1".to_string(),
        medium_id: Some("f4ffaf26-ce12-5be0-9f99-e249743679ac".to_string()),
        name: "Northwind Traders".to_string(),
    })
}
