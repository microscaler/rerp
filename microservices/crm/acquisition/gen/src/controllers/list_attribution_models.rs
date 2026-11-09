// User-owned controller for handler 'list_attribution_models'.

use crate::handlers::list_attribution_models::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AttributionModel;

#[handler(ListAttributionModelsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "name": "Northwind Traders"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
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

    HttpJson::ok(Response(vec![serde_json::from_value::<AttributionModel>(
        serde_json::json!({"name":"Northwind Traders"}),
    )
    .unwrap_or_default()]))
}
