// User-owned controller for handler 'run_workflow'.

use crate::handlers::run_workflow::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RunWorkflowController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "actions_ran": 3,
    //   "errors": [
    //     "Errors"
    //   ],
    //   "executed": false,
    //   "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "actions_ran": 3,
  "errors": [
    "Errors"
  ],
  "executed": false,
  "workflow_id": "7008f5a1-8ead-5a4f-8cd3-a3deab21a548"
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        actions_ran: 3,
        errors: vec!["Errors".to_string()],
        executed: false,
        workflow_id: "7008f5a1-8ead-5a4f-8cd3-a3deab21a548".to_string(),
    })
}
