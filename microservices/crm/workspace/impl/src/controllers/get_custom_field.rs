// Implementation stub for handler 'get_custom_field'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::get_custom_field::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_workspace_gen::handlers::types::FieldType;

#[handler(GetCustomFieldController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        created_at: None,              // TODO: Set from your business logic
        created_by: None,              // TODO: Set from your business logic
        default_value: None,           // TODO: Set from your business logic
        entity: "contact".to_string(), // TODO: Set from your business logic
        group: None,                   // TODO: Set from your business logic
        hint: None,                    // TODO: Set from your business logic
        id: "9851eff3-bcf4-59e1-9c43-82ba49bdc08c".to_string(), // TODO: Set from your business logic
        key: "plan".to_string(),   // TODO: Set from your business logic
        label: "Plan".to_string(), // TODO: Set from your business logic
        merge_field: None,         // TODO: Set from your business logic
        options: None,             // TODO: Set from your business logic
        personalization_tag: None, // TODO: Set from your business logic
        required: None,            // TODO: Set from your business logic
        required_at_stage: None,   // TODO: Set from your business logic
        searchable: None,          // TODO: Set from your business logic
        sequence: None,            // TODO: Set from your business logic
        show_on_card: None,        // TODO: Set from your business logic
        r#type: serde_json::from_value::<FieldType>(serde_json::Value::String(
            "SELECT".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        visible: false,            // TODO: Set from your business logic
    })
}
