// Implementation stub for handler 'create_suppression'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_audience_gen::handlers::create_suppression::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_audience_gen::handlers::types::Channel;
#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_audience_gen::handlers::types::SuppressionReason;

#[handler(CreateSuppressionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let address = req.inner.address;// let channel = req.inner.channel;// let note = req.inner.note;// let x_workspace_id = req.inner.x_workspace_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        address: "daniel.kim@wideworld.example".to_string(), // TODO: Set from your business logic
        channel: None,                                       // TODO: Set from your business logic
        created_at: "2026-10-28T16:00:00Z".to_string(),      // TODO: Set from your business logic
        id: "fc58b14d-e13f-5d43-bad7-0a86f5d330c6".to_string(), // TODO: Set from your business logic
        note: None, // TODO: Set from your business logic
        reason: serde_json::from_value::<SuppressionReason>(serde_json::Value::String(
            "HARD_BOUNCE".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        source: None, // TODO: Set from your business logic
        source_send_id: None, // TODO: Set from your business logic
    })
}
