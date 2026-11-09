// User-owned controller for handler 'list_field_groups'.

use crate::handlers::list_field_groups::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FieldGroup;

#[handler(ListFieldGroupsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "entity": "deal",
    //       "id": "25f0a77a-7b4a-5920-9ff0-a8c43e04063c",
    //       "name": "Deal information",
    //       "sequence": 0
    //     },
    //     {
    //       "entity": "deal",
    //       "id": "d0d7d178-94f3-57b2-9d7b-893c784b0f0d",
    //       "name": "Commercials",
    //       "sequence": 1
    //     },
    //     {
    //       "entity": "contact",
    //       "id": "dfab8875-e18d-5385-911b-0ab29df3e038",
    //       "name": "Profile",
    //       "sequence": 0
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 5
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "entity": "deal",
      "id": "25f0a77a-7b4a-5920-9ff0-a8c43e04063c",
      "name": "Deal information",
      "sequence": 0
    },
    {
      "entity": "deal",
      "id": "d0d7d178-94f3-57b2-9d7b-893c784b0f0d",
      "name": "Commercials",
      "sequence": 1
    },
    {
      "entity": "contact",
      "id": "dfab8875-e18d-5385-911b-0ab29df3e038",
      "name": "Profile",
      "sequence": 0
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 5
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<FieldGroup>(serde_json::json!({"entity":"deal","id":"25f0a77a-7b4a-5920-9ff0-a8c43e04063c","name":"Deal information","sequence":0})).unwrap_or_default(), serde_json::from_value::<FieldGroup>(serde_json::json!({"entity":"deal","id":"d0d7d178-94f3-57b2-9d7b-893c784b0f0d","name":"Commercials","sequence":1})).unwrap_or_default(), serde_json::from_value::<FieldGroup>(serde_json::json!({"entity":"contact","id":"dfab8875-e18d-5385-911b-0ab29df3e038","name":"Profile","sequence":0})).unwrap_or_default()],limit: 25,page: 1,total: 5,
    })
}
