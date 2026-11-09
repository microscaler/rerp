// User-owned controller for handler 'get_workflow_setup'.

use crate::handlers::get_workflow_setup::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::RecipeParameter;

#[handler(GetWorkflowSetupController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "parameters": [
    //     {
    //       "key": "snippet.signature",
    //       "kind": "LIST",
    //       "label": "Primary"
    //     }
    //   ],
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "parameters": [
    {
      "key": "snippet.signature",
      "kind": "LIST",
      "label": "Primary"
    }
  ],
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
        parameters: vec![serde_json::from_value::<RecipeParameter>(
            serde_json::json!({"key":"snippet.signature","kind":"LIST","label":"Primary"}),
        )
        .unwrap_or_default()],
        total: Some(2),
    })
}
