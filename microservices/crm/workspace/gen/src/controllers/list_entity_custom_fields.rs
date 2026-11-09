// User-owned controller for handler 'list_entity_custom_fields'.

use crate::handlers::list_entity_custom_fields::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CustomField;

#[handler(ListEntityCustomFieldsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "entity": "Entity",
    //       "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
    //       "key": "snippet.signature",
    //       "label": "Primary",
    //       "type": "STRING",
    //       "visible": false
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
      "entity": "Entity",
      "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
      "key": "snippet.signature",
      "label": "Primary",
      "type": "STRING",
      "visible": false
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
        items: vec![serde_json::from_value::<CustomField>(serde_json::json!({"entity":"Entity","id":"94b293ed-258b-5091-8e79-75d2ac471f25","key":"snippet.signature","label":"Primary","type":"STRING","visible":false})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
