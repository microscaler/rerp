// User-owned controller for handler 'update_postal_address'.

use crate::handlers::update_postal_address::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdatePostalAddressController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "address": "Address",
    //   "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //   "is_default": true,
    //   "name": "Northwind Traders",
    //   "used_by": [
    //     "Used by"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "address": "Address",
  "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
  "is_default": true,
  "name": "Northwind Traders",
  "used_by": [
    "Used by"
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        address: "Address".to_string(),
        id: "3eb009b7-a99d-5a4c-956a-99e11d8dbb76".to_string(),
        is_default: Some(true),
        name: "Northwind Traders".to_string(),
        used_by: Some(vec!["Used by".to_string()]),
    })
}
