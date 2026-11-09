// User-owned controller for handler 'list_workspaces'.

use crate::handlers::list_workspaces::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Workspace;

#[handler(ListWorkspacesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
    //       "name": "Northwind Traders",
    //       "organisation_id": "20df1243-e229-5084-b57d-e48d795731f8"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
      "name": "Northwind Traders",
      "organisation_id": "20df1243-e229-5084-b57d-e48d795731f8"
    }
  ],
  "limit": 25,
  "page": 1,
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
        items: vec![serde_json::from_value::<Workspace>(serde_json::json!({"id":"94b293ed-258b-5091-8e79-75d2ac471f25","name":"Northwind Traders","organisation_id":"20df1243-e229-5084-b57d-e48d795731f8"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
