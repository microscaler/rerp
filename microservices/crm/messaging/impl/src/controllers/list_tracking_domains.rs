// Implementation stub for handler 'list_tracking_domains'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::list_tracking_domains::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_messaging_gen::handlers::types::TrackingDomain;

#[handler(ListTrackingDomainsController)]
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
        items: vec![serde_json::from_value::<TrackingDomain>(serde_json::json!({"domain":"mail.acme.example","id":"7b8f77a3-8b75-51bb-a3b9-b7267f501814","last_checked_at":"2026-11-05T15:56:00Z","records":[{"name":"mail.acme.example","type":"CNAME","value":"edge.crm.example.net","verified":true},{"name":"_crm-verify.mail.acme.example","type":"TXT","value":"crm-verify=7c1e…","verified":true}],"status":"VERIFIED","tls":true})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 1, // TODO: Set from your business logic
    })
}
