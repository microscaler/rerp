// Implementation stub for handler 'get_attribution_overview'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::get_attribution_overview::{Request, Response};

#[handler(GetAttributionOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let from = req.inner.from;// let to = req.inner.to;// let group_by = req.inner.group_by;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        funnel: None, // TODO: Set from your business logic
        rows: vec![serde_json::from_str::<serde_json::Value>(r#"{"contacts":1104,"converted":9,"key":"google / cpc","leads":612,"revenue":"38400.00"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"contacts":902,"converted":6,"key":"linkedin / social","leads":418,"revenue":"41000.00"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"contacts":0,"converted":7,"key":"newsletter / email","leads":301,"revenue":"12140.00"}"#).unwrap_or(serde_json::Value::Null)], // TODO: Set from your business logic
    })
}
