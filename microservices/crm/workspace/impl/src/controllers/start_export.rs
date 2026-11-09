// Implementation stub for handler 'start_export'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::start_export::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_workspace_gen::handlers::types::JobEntity;
#[allow(unused_imports)]
use rerp_crm_workspace_gen::handlers::types::JobKind;
#[allow(unused_imports)]
use rerp_crm_workspace_gen::handlers::types::JobOptions;
#[allow(unused_imports)]
use rerp_crm_workspace_gen::handlers::types::JobStatus;

#[handler(StartExportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let entity = req.inner.entity;// let notify_email = req.inner.notify_email;// let options = req.inner.options;// let x_workspace_id = req.inner.x_workspace_id;// let idempotency_key = req.inner.idempotency_key;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        created_rows: None, // TODO: Set from your business logic
        entity: serde_json::from_value::<JobEntity>(serde_json::Value::String(
            "CONTACTS".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        error_rows: None,   // TODO: Set from your business logic
        file_name: None,    // TODO: Set from your business logic
        finished_at: None,  // TODO: Set from your business logic
        id: "9b732034-b874-5185-911e-fd4355d5e7f0".to_string(), // TODO: Set from your business logic
        integration_id: None, // TODO: Set from your business logic
        kind: serde_json::from_value::<JobKind>(serde_json::Value::String("EXPORT".to_string()))
            .unwrap_or_default(), // TODO: Set from your business logic
        message: None,        // TODO: Set from your business logic
        options: None,        // TODO: Set from your business logic
        processed_rows: None, // TODO: Set from your business logic
        progress_pct: None,   // TODO: Set from your business logic
        queued_at: "2026-11-05T16:00:00Z".to_string(), // TODO: Set from your business logic
        requested_by: "alex".to_string(), // TODO: Set from your business logic
        requested_by_name: None, // TODO: Set from your business logic
        result_expires_at: None, // TODO: Set from your business logic
        result_url: None,     // TODO: Set from your business logic
        skipped_rows: None,   // TODO: Set from your business logic
        source: None,         // TODO: Set from your business logic
        started_at: None,     // TODO: Set from your business logic
        status: serde_json::from_value::<JobStatus>(serde_json::Value::String(
            "QUEUED".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        total_rows: None,     // TODO: Set from your business logic
        updated_rows: None,   // TODO: Set from your business logic
    })
}
