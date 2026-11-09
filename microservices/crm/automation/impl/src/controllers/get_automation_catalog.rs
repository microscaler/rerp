// Implementation stub for handler 'get_automation_catalog'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_automation_gen::handlers::get_automation_catalog::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_automation_gen::handlers::types::CatalogEntry;

#[handler(GetAutomationCatalogController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        actions: vec![serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Sending","kind":"ACTION","label":"Send email","type":"SEND_MESSAGE"})).unwrap_or_default(), serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Sending","kind":"ACTION","label":"Send SMS","type":"SEND_SMS"})).unwrap_or_default(), serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Sending","kind":"ACTION","label":"Send 1:1 email","type":"SEND_1TO1_EMAIL"})).unwrap_or_default()], // TODO: Set from your business logic
        triggers: vec![serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Behaviors","label":"On a schedule","type":"TIME_BASED"})).unwrap_or_default(), serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Email & messages","label":"Subscribes to a list","type":"SUBSCRIBED_TO_LIST"})).unwrap_or_default(), serde_json::from_value::<CatalogEntry>(serde_json::json!({"group":"Email & messages","label":"Unsubscribes from a list","type":"UNSUBSCRIBED_FROM_LIST"})).unwrap_or_default()], // TODO: Set from your business logic
    })
}
