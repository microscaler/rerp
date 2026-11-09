// Implementation stub for handler 'update_member'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::update_member::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_workspace_gen::handlers::types::WorkspaceRole;

#[handler(UpdateMemberController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let role = req.inner.role;// let id = req.inner.id;// let user_id = req.inner.user_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        added_at: None,       // TODO: Set from your business logic
        email: None,          // TODO: Set from your business logic
        last_active_at: None, // TODO: Set from your business logic
        name: None,           // TODO: Set from your business logic
        role: serde_json::from_value::<WorkspaceRole>(serde_json::Value::String(
            "owner".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        user_id: "alex".to_string(), // TODO: Set from your business logic
    })
}
