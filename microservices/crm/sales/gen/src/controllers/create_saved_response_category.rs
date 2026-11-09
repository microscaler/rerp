// User-owned controller for handler 'create_saved_response_category'.

use crate::handlers::create_saved_response_category::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateSavedResponseCategoryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
    //   "name": "Sales"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
  "name": "Sales"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        id: "113ca5aa-e3f2-576a-b6b5-79dbb715308e".to_string(),
        name: "Sales".to_string(),
        response_count: Some(42),
    })
}
