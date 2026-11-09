// Implementation stub for handler 'update_conversion_definition'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::update_conversion_definition::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_acquisition_gen::handlers::types::ConversionTrigger;

#[handler(UpdateConversionDefinitionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let currency = req.inner.currency;// let name = req.inner.name;// let once_per_contact = req.inner.once_per_contact;// let trigger = req.inner.trigger;// let value = req.inner.value;// let value_mode = req.inner.value_mode;// let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        count_30d: None,  // TODO: Set from your business logic
        created_at: None, // TODO: Set from your business logic
        currency: None,   // TODO: Set from your business logic
        id: "e67555dd-33f4-5fee-9da3-a019895219b1".to_string(), // TODO: Set from your business logic
        last_at: None,                         // TODO: Set from your business logic
        name: "Northwind Traders".to_string(), // TODO: Set from your business logic
        once_per_contact: None,                // TODO: Set from your business logic
        revenue_30d: None,                     // TODO: Set from your business logic
        total: None,                           // TODO: Set from your business logic
        trigger: serde_json::from_value::<ConversionTrigger>(
            serde_json::json!({"kind":"FORM_SUBMITTED","ref":"Ref"}),
        )
        .unwrap_or_default(), // TODO: Set from your business logic
        value: None,                           // TODO: Set from your business logic
        value_mode: None,                      // TODO: Set from your business logic
    })
}
