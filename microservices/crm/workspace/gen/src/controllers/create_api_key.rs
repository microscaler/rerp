// User-owned controller for handler 'create_api_key'.

use crate::handlers::create_api_key::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateApiKeyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "id": "b90438d1-e134-5543-919f-19864d0919d3",
    //   "name": "Website (public tracking key)"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "id": "b90438d1-e134-5543-919f-19864d0919d3",
  "name": "Website (public tracking key)"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        id: Some("b90438d1-e134-5543-919f-19864d0919d3".to_string()),
        key: Some("example".to_string()),
        key_hash: Some("example".to_string()),
        name: Some("Website (public tracking key)".to_string()),
    })
}
