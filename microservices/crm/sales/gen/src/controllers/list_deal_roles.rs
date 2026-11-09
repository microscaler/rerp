// User-owned controller for handler 'list_deal_roles'.

use crate::handlers::list_deal_roles::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealRole;

#[handler(ListDealRolesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "description": "Default role for the deal's first contact",
    //       "id": "c0fd9836-bb8e-5080-b832-50c4843b3e9f",
    //       "title": "Primary contact",
    //       "used_on": 38
    //     },
    //     {
    //       "description": "Signs off budget",
    //       "id": "13e07af8-0ff7-55cb-92ef-95d54cd5b873",
    //       "title": "Decision maker",
    //       "used_on": 14
    //     },
    //     {
    //       "description": "Internal advocate",
    //       "id": "77c6efec-76dd-550c-bf81-5771cb2b67ac",
    //       "title": "Champion",
    //       "used_on": 11
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 4
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "description": "Default role for the deal's first contact",
      "id": "c0fd9836-bb8e-5080-b832-50c4843b3e9f",
      "title": "Primary contact",
      "used_on": 38
    },
    {
      "description": "Signs off budget",
      "id": "13e07af8-0ff7-55cb-92ef-95d54cd5b873",
      "title": "Decision maker",
      "used_on": 14
    },
    {
      "description": "Internal advocate",
      "id": "77c6efec-76dd-550c-bf81-5771cb2b67ac",
      "title": "Champion",
      "used_on": 11
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 4
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<DealRole>(serde_json::json!({"description":"Default role for the deal's first contact","id":"c0fd9836-bb8e-5080-b832-50c4843b3e9f","title":"Primary contact","used_on":38})).unwrap_or_default(), serde_json::from_value::<DealRole>(serde_json::json!({"description":"Signs off budget","id":"13e07af8-0ff7-55cb-92ef-95d54cd5b873","title":"Decision maker","used_on":14})).unwrap_or_default(), serde_json::from_value::<DealRole>(serde_json::json!({"description":"Internal advocate","id":"77c6efec-76dd-550c-bf81-5771cb2b67ac","title":"Champion","used_on":11})).unwrap_or_default()],limit: 25,page: 1,total: 4,
    })
}
