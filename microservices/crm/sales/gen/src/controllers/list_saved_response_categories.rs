// User-owned controller for handler 'list_saved_response_categories'.

use crate::handlers::list_saved_response_categories::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::SavedResponseCategory;

#[handler(ListSavedResponseCategoriesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
    //       "name": "Sales"
    //     },
    //     {
    //       "id": "cab98488-f673-5409-9af6-81de2f44a796",
    //       "name": "Scheduling"
    //     },
    //     {
    //       "id": "518ebcf2-d9c3-578e-8327-cefcd54eed7e",
    //       "name": "Customer success"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
      "name": "Sales"
    },
    {
      "id": "cab98488-f673-5409-9af6-81de2f44a796",
      "name": "Scheduling"
    },
    {
      "id": "518ebcf2-d9c3-578e-8327-cefcd54eed7e",
      "name": "Customer success"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 3
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<SavedResponseCategory>(serde_json::json!({"id":"113ca5aa-e3f2-576a-b6b5-79dbb715308e","name":"Sales"})).unwrap_or_default(), serde_json::from_value::<SavedResponseCategory>(serde_json::json!({"id":"cab98488-f673-5409-9af6-81de2f44a796","name":"Scheduling"})).unwrap_or_default(), serde_json::from_value::<SavedResponseCategory>(serde_json::json!({"id":"518ebcf2-d9c3-578e-8327-cefcd54eed7e","name":"Customer success"})).unwrap_or_default()],limit: 25,page: 1,total: 3,
    })
}
