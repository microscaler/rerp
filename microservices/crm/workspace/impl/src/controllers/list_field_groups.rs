// Implementation stub for handler 'list_field_groups'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::list_field_groups::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_workspace_gen::handlers::types::FieldGroup;

#[handler(ListFieldGroupsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let entity = req.inner.entity;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<FieldGroup>(serde_json::json!({"entity":"deal","id":"25f0a77a-7b4a-5920-9ff0-a8c43e04063c","name":"Deal information","sequence":0})).unwrap_or_default(), serde_json::from_value::<FieldGroup>(serde_json::json!({"entity":"deal","id":"d0d7d178-94f3-57b2-9d7b-893c784b0f0d","name":"Commercials","sequence":1})).unwrap_or_default(), serde_json::from_value::<FieldGroup>(serde_json::json!({"entity":"contact","id":"dfab8875-e18d-5385-911b-0ab29df3e038","name":"Profile","sequence":0})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 5, // TODO: Set from your business logic
    })
}
