// User-owned controller for handler 'list_rules'.

use crate::handlers::list_rules::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Rule;

#[handler(ListRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "condition_field": "Condition field",
    //       "condition_operator": "EQUALS",
    //       "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //       "is_active": true,
    //       "sequence": 1,
    //       "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "condition_field": "Condition field",
      "condition_operator": "EQUALS",
      "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
      "is_active": true,
      "sequence": 1,
      "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Rule>(serde_json::json!({"condition_field":"Condition field","condition_operator":"EQUALS","id":"c9e18116-9fea-5e6e-ac32-52580d2450b8","is_active":true,"sequence":1,"workflow_id":"7008f5a1-8ead-5a4f-8cd3-a3deab21a548"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
