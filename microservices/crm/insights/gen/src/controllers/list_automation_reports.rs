// User-owned controller for handler 'list_automation_reports'.

use crate::handlers::list_automation_reports::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AutomationListRow;

#[handler(ListAutomationReportsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contacts_entered": 3,
    //       "name": "Northwind Traders",
    //       "workflow_id": "36523d6f-c5d7-562b-8bd5-7cb5480e5d2e"
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
      "contacts_entered": 3,
      "name": "Northwind Traders",
      "workflow_id": "36523d6f-c5d7-562b-8bd5-7cb5480e5d2e"
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
        items: vec![serde_json::from_value::<AutomationListRow>(serde_json::json!({"contacts_entered":3,"name":"Northwind Traders","workflow_id":"36523d6f-c5d7-562b-8bd5-7cb5480e5d2e"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
