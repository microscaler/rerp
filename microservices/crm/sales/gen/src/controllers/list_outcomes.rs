// User-owned controller for handler 'list_outcomes'.

use crate::handlers::list_outcomes::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Outcome;

#[handler(ListOutcomesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "id": "0a7437f6-0e3e-51ae-a9fd-4a44b900f91e",
    //       "label": "Reached",
    //       "sentiment": "POSITIVE"
    //     },
    //     {
    //       "id": "ca29541d-dde8-5a9e-818d-b0717df850c9",
    //       "label": "Replied",
    //       "sentiment": "POSITIVE"
    //     },
    //     {
    //       "id": "822890d5-c8b4-5558-a84c-60e98767732f",
    //       "label": "Held",
    //       "sentiment": "POSITIVE"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 12
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "id": "0a7437f6-0e3e-51ae-a9fd-4a44b900f91e",
      "label": "Reached",
      "sentiment": "POSITIVE"
    },
    {
      "id": "ca29541d-dde8-5a9e-818d-b0717df850c9",
      "label": "Replied",
      "sentiment": "POSITIVE"
    },
    {
      "id": "822890d5-c8b4-5558-a84c-60e98767732f",
      "label": "Held",
      "sentiment": "POSITIVE"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 12
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Outcome>(serde_json::json!({"id":"0a7437f6-0e3e-51ae-a9fd-4a44b900f91e","label":"Reached","sentiment":"POSITIVE"})).unwrap_or_default(), serde_json::from_value::<Outcome>(serde_json::json!({"id":"ca29541d-dde8-5a9e-818d-b0717df850c9","label":"Replied","sentiment":"POSITIVE"})).unwrap_or_default(), serde_json::from_value::<Outcome>(serde_json::json!({"id":"822890d5-c8b4-5558-a84c-60e98767732f","label":"Held","sentiment":"POSITIVE"})).unwrap_or_default()],limit: 25,page: 1,total: 12,
    })
}
