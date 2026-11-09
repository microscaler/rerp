// User-owned controller for handler 'get_automation_goals'.

use crate::handlers::get_automation_goals::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AutomationGoalRow;

#[handler(GetAutomationGoalsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "goals": [
    //     {
    //       "achieved_rate": 0.23,
    //       "avg_seconds_to_achieve": 207360,
    //       "condition": "Site event trial_activated occurred",
    //       "contacts_achieved": 377,
    //       "contacts_skipped": 1025,
    //       "contacts_waiting": 238,
    //       "name": "Activated the product",
    //       "step_id": "047a18d8-3dbe-5b48-8733-4d69800e8319"
    //     }
    //   ],
    //   "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "goals": [
    {
      "achieved_rate": 0.23,
      "avg_seconds_to_achieve": 207360,
      "condition": "Site event trial_activated occurred",
      "contacts_achieved": 377,
      "contacts_skipped": 1025,
      "contacts_waiting": 238,
      "name": "Activated the product",
      "step_id": "047a18d8-3dbe-5b48-8733-4d69800e8319"
    }
  ],
  "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        goals: vec![serde_json::from_value::<AutomationGoalRow>(serde_json::json!({"achieved_rate":0.23,"avg_seconds_to_achieve":207360,"condition":"Site event trial_activated occurred","contacts_achieved":377,"contacts_skipped":1025,"contacts_waiting":238,"name":"Activated the product","step_id":"047a18d8-3dbe-5b48-8733-4d69800e8319"})).unwrap_or_default()],workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(),
    })
}
