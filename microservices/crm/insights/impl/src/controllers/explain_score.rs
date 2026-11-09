// Implementation stub for handler 'explain_score'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::explain_score::{Request, Response};

#[handler(ExplainScoreController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let contact_id = req.inner.contact_id;// let deal_id = req.inner.deal_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        band: None,         // TODO: Set from your business logic
        contact_id: None,   // TODO: Set from your business logic
        contact_name: None, // TODO: Set from your business logic
        entity_id: "d7976b34-ac30-5901-8042-56e3d81193ab".to_string(), // TODO: Set from your business logic
        entity_name: None, // TODO: Set from your business logic
        factors: vec![
            serde_json::from_str::<serde_json::Value>(
                r#"{"label":"Event: activated","points":25}"#,
            )
            .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(
                r#"{"label":"Plan is Business","points":20}"#,
            )
            .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(
                r#"{"label":"Visited /pricing","points":30}"#,
            )
            .unwrap_or(serde_json::Value::Null),
        ], // TODO: Set from your business logic
        score: 86,         // TODO: Set from your business logic
        score_id: "cef670be-5b12-52e7-8232-19a36fb64702".to_string(), // TODO: Set from your business logic
    })
}
