// User-owned controller for handler 'create_field_group'.

use crate::handlers::create_field_group::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateFieldGroupController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "entity": "deal",
    //   "id": "25f0a77a-7b4a-5920-9ff0-a8c43e04063c",
    //   "name": "Deal information",
    //   "sequence": 0
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "entity": "deal",
  "id": "25f0a77a-7b4a-5920-9ff0-a8c43e04063c",
  "name": "Deal information",
  "sequence": 0
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        entity: "deal".to_string(),
        id: "25f0a77a-7b4a-5920-9ff0-a8c43e04063c".to_string(),
        name: "Deal information".to_string(),
        sequence: Some(0),
    })
}
