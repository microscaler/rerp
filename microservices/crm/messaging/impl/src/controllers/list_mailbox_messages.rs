// Implementation stub for handler 'list_mailbox_messages'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::list_mailbox_messages::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_messaging_gen::handlers::types::MailboxMessage;

#[handler(ListMailboxMessagesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let contact_id = req.inner.contact_id;// let deal_id = req.inner.deal_id;// let direction = req.inner.direction;// let since = req.inner.since;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<MailboxMessage>(serde_json::json!({"clicks":0,"contact_ids":["af512c4f-0beb-50b6-bf61-7a9d8bd417b5"],"contact_name":"Priya Natarajan","deal_ids":["63a6661f-5860-50ed-8697-f28d29a3917f"],"deal_title":"Northwind · 48 → 120 seats","direction":"INBOUND","from":"priya.natarajan@northwindtraders.example","id":"8d5a1935-b54e-5567-ac56-e51e2656fa19","mailbox_id":"22788e76-5b04-5e9c-97a4-49cade135ff3","opens":0,"sent_at":"2026-11-05T11:00:00Z","subject":"Re: Proposal for 120 seats","to":["alex@acme.example"]})).unwrap_or_default(), serde_json::from_value::<MailboxMessage>(serde_json::json!({"clicks":1,"contact_ids":["af512c4f-0beb-50b6-bf61-7a9d8bd417b5"],"contact_name":"Priya Natarajan","deal_ids":["63a6661f-5860-50ed-8697-f28d29a3917f"],"deal_title":"Northwind · 48 → 120 seats","direction":"OUTBOUND","from":"alex@acme.example","id":"8e29079a-9ff4-5a6e-b030-f098c14b3148","mailbox_id":"22788e76-5b04-5e9c-97a4-49cade135ff3","opens":2,"sent_at":"2026-11-05T05:00:00Z","subject":"Proposal for 120 seats","to":["priya.natarajan@northwindtraders.example"]})).unwrap_or_default(), serde_json::from_value::<MailboxMessage>(serde_json::json!({"clicks":0,"contact_ids":["7ed65e2d-0bd7-5e4e-9071-f4dac3456426"],"contact_name":"Tom Okafor","deal_ids":[],"deal_title":"Deal title","direction":"INBOUND","from":"tom.okafor@fabrikam.example","id":"7882e22e-5ff5-5c69-ac76-7fe91510e0b1","mailbox_id":"22788e76-5b04-5e9c-97a4-49cade135ff3","opens":0,"sent_at":"2026-11-05T06:00:00Z","subject":"SSO docs","to":["alex@acme.example"]})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 4, // TODO: Set from your business logic
    })
}
