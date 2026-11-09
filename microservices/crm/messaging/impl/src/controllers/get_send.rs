// Implementation stub for handler 'get_send'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::get_send::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::Channel;
#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::MessageEvent;
#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::Provider;
#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::SendStatus;

#[handler(GetSendController)]
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
        campaign_id: None,   // TODO: Set from your business logic
        campaign_name: None, // TODO: Set from your business logic
        channel: serde_json::from_value::<Channel>(serde_json::Value::String("EMAIL".to_string()))
            .unwrap_or_default(), // TODO: Set from your business logic
        clicked_at: None,    // TODO: Set from your business logic
        clicked_link: None,  // TODO: Set from your business logic
        contact_id: "c891a339-996d-56b9-a022-244c570f48c9".to_string(), // TODO: Set from your business logic
        contact_name: None, // TODO: Set from your business logic
        created_at: "2026-11-05T16:00:00Z".to_string(), // TODO: Set from your business logic
        enrolment_id: None, // TODO: Set from your business logic
        error: None,        // TODO: Set from your business logic
        events: None,       // TODO: Set from your business logic
        id: "3eb009b7-a99d-5a4c-956a-99e11d8dbb76".to_string(), // TODO: Set from your business logic
        opened_at: None,           // TODO: Set from your business logic
        provider: None,            // TODO: Set from your business logic
        provider_message_id: None, // TODO: Set from your business logic
        queued: None,              // TODO: Set from your business logic
        status: serde_json::from_value::<SendStatus>(serde_json::Value::String(
            "QUEUED".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        subject: None,             // TODO: Set from your business logic
        template_id: None,         // TODO: Set from your business logic
        template_version: None,    // TODO: Set from your business logic
        to: None,                  // TODO: Set from your business logic
        variant_key: None,         // TODO: Set from your business logic
        workflow_id: None,         // TODO: Set from your business logic
    })
}
