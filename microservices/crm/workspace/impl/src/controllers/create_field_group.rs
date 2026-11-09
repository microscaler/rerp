// Implementation stub for handler 'create_field_group'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::create_field_group::{Request, Response};

#[handler(CreateFieldGroupController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let entity = req.inner.entity;// let name = req.inner.name;// let sequence = req.inner.sequence;// let x_workspace_id = req.inner.x_workspace_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        entity: "deal".to_string(), // TODO: Set from your business logic
        id: "25f0a77a-7b4a-5920-9ff0-a8c43e04063c".to_string(), // TODO: Set from your business logic
        name: "Deal information".to_string(), // TODO: Set from your business logic
        sequence: None,                       // TODO: Set from your business logic
    })
}
