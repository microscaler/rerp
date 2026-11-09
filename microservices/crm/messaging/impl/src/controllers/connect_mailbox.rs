// Implementation stub for handler 'connect_mailbox'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::connect_mailbox::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_messaging_gen::handlers::types::ImapConfig;
#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_messaging_gen::handlers::types::MailboxProvider;
#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_messaging_gen::handlers::types::MailboxStatus;

#[handler(ConnectMailboxController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let address = req.inner.address;// let display_name = req.inner.display_name;// let imap = req.inner.imap;// let log_to_deals = req.inner.log_to_deals;// let provider = req.inner.provider;// let redirect_uri = req.inner.redirect_uri;// let sync_enabled = req.inner.sync_enabled;// let sync_scope = req.inner.sync_scope;// let sync_since = req.inner.sync_since;// let track_clicks = req.inner.track_clicks;// let track_opens = req.inner.track_opens;// let x_workspace_id = req.inner.x_workspace_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        address: "alex@acme.example".to_string(), // TODO: Set from your business logic
        authorize_url: None,                      // TODO: Set from your business logic
        created_at: None,                         // TODO: Set from your business logic
        display_name: None,                       // TODO: Set from your business logic
        id: "22788e76-5b04-5e9c-97a4-49cade135ff3".to_string(), // TODO: Set from your business logic
        imap: None,         // TODO: Set from your business logic
        last_error: None,   // TODO: Set from your business logic
        last_sync_at: None, // TODO: Set from your business logic
        log_to_deals: None, // TODO: Set from your business logic
        provider: serde_json::from_value::<MailboxProvider>(serde_json::Value::String(
            "GOOGLE".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        shared_with_user_ids: None, // TODO: Set from your business logic
        signature_html: None, // TODO: Set from your business logic
        status: serde_json::from_value::<MailboxStatus>(serde_json::Value::String(
            "CONNECTED".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        sync_enabled: true, // TODO: Set from your business logic
        sync_scope: None,   // TODO: Set from your business logic
        sync_since: None,   // TODO: Set from your business logic
        track_clicks: None, // TODO: Set from your business logic
        track_opens: None,  // TODO: Set from your business logic
        updated_at: None,   // TODO: Set from your business logic
        user_id: "alex".to_string(), // TODO: Set from your business logic
        user_name: None,    // TODO: Set from your business logic
    })
}
