// User-owned controller for handler 'list_workflow_recipes'.

use crate::handlers::list_workflow_recipes::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WorkflowRecipe;

#[handler(ListWorkflowRecipesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "category": "WELCOME",
    //       "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //       "name": "Northwind Traders"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "category": "WELCOME",
      "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
      "name": "Northwind Traders"
    }
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response(vec![serde_json::from_value::<WorkflowRecipe>(serde_json::json!({"category":"WELCOME","id":"c9e18116-9fea-5e6e-ac32-52580d2450b8","name":"Northwind Traders"})).unwrap_or_default()]))
}
