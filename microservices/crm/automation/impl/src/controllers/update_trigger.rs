// Implementation stub for handler 'update_trigger'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_automation_gen::handlers::update_trigger::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_automation_gen::handlers::types::TriggerType;

#[handler(UpdateTriggerController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let enabled = req.inner.enabled;// let entity = req.inner.entity;// let field = req.inner.field;// let schedule = req.inner.schedule;// let r#type = req.inner.r#type;// let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        created_at: None,             // TODO: Set from your business logic
        enabled: true,                // TODO: Set from your business logic
        entity: "Entity".to_string(), // TODO: Set from your business logic
        field: None,                  // TODO: Set from your business logic
        id: "c9e18116-9fea-5e6e-ac32-52580d2450b8".to_string(), // TODO: Set from your business logic
        last_fired: None, // TODO: Set from your business logic
        schedule: None,   // TODO: Set from your business logic
        r#type: serde_json::from_value::<TriggerType>(serde_json::Value::String(
            "STAGE_CHANGE".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        workflow_id: "7008f5a1-8ead-5a4f-8cd3-a3deab21a548".to_string(), // TODO: Set from your business logic
    })
}
