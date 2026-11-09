// Implementation stub for handler 'list_conversion_definitions'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::list_conversion_definitions::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_acquisition_gen::handlers::types::ConversionDefinition;

#[handler(ListConversionDefinitionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<ConversionDefinition>(serde_json::json!({"count_30d":2140,"created_at":"2026-06-08T16:00:00Z","currency":"EUR","id":"30eefab7-ff42-50cf-85bd-3a974b1ce548","last_at":"2026-11-05T15:51:00Z","name":"Trial signup","once_per_contact":true,"revenue_30d":"0.00","total":6114,"trigger":{"kind":"FORM_SUBMITTED","ref":"Free trial"},"value":"0.00","value_mode":"NONE"})).unwrap_or_default(), serde_json::from_value::<ConversionDefinition>(serde_json::json!({"count_30d":1312,"created_at":"2026-06-08T16:00:00Z","currency":"EUR","id":"473f4a5c-dcac-59e5-b889-2eb270d9db5d","last_at":"2026-11-05T15:58:00Z","name":"Activated","once_per_contact":true,"revenue_30d":"52480.00","total":3988,"trigger":{"kind":"FORM_SUBMITTED","ref":"activated"},"value":"40.00","value_mode":"FIXED"})).unwrap_or_default(), serde_json::from_value::<ConversionDefinition>(serde_json::json!({"count_30d":58,"created_at":"2026-06-08T16:00:00Z","currency":"EUR","id":"62363f71-61de-5e1b-ab25-896ce248a502","last_at":"2026-11-05T13:00:00Z","name":"Booked a demo","once_per_contact":true,"revenue_30d":"14500.00","total":412,"trigger":{"kind":"FORM_SUBMITTED","ref":"Demo request"},"value":"250.00","value_mode":"FIXED"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 5, // TODO: Set from your business logic
    })
}
