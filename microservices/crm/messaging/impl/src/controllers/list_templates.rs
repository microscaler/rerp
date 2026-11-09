// Implementation stub for handler 'list_templates'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::list_templates::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_messaging_gen::handlers::types::Template;

#[handler(ListTemplatesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;// let channel = req.inner.channel;// let kind = req.inner.kind;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Template>(serde_json::json!({"body":"Hi {{contact.first_name}},\n\nWelcome · day 0: what you can do today.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})","body_format":"MARKDOWN","channel":"EMAIL","click_rate":0.14,"created_at":"2026-07-08T16:00:00Z","id":"736685cb-324b-572c-b287-deb656a4d8de","is_archived":false,"kind":"AUTOMATION","language_code":"en","merge_fields":["contact.first_name","sender.postal_address","links.unsubscribe"],"name":"Welcome · day 0: what you can do today","open_rate":0.62,"preheader":"","priority":"NORMAL","sent_30d":1640,"subject":"Welcome to {{sender.name}}","unsub_30d":2,"updated_at":"2026-10-16T16:00:00Z","used_by":[{"step":1,"workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"}],"version":3,"versions":[{"at":"2026-10-19T16:00:00Z","version":3},{"at":"2026-09-09T16:00:00Z","version":2},{"at":"2026-07-31T16:00:00Z","version":1}]})).unwrap_or_default(), serde_json::from_value::<Template>(serde_json::json!({"body":"Hi {{contact.first_name}},\n\nWelcome · day 2: three things to set up.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})","body_format":"MARKDOWN","channel":"EMAIL","click_rate":0.11,"created_at":"2026-07-13T16:00:00Z","id":"f248dd12-4837-5fde-aa51-2fba5a0e8227","is_archived":false,"kind":"AUTOMATION","language_code":"en","merge_fields":["contact.first_name","sender.postal_address","links.unsubscribe"],"name":"Welcome · day 2: three things to set up","open_rate":0.51,"preheader":"","priority":"NORMAL","sent_30d":1004,"subject":"Three things worth setting up","unsub_30d":1,"updated_at":"2026-10-17T16:00:00Z","used_by":[{"step":"4 (yes branch)","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"}],"version":2,"versions":[{"at":"2026-10-18T16:00:00Z","version":2},{"at":"2026-09-08T16:00:00Z","version":1}]})).unwrap_or_default(), serde_json::from_value::<Template>(serde_json::json!({"body":"Hi {{contact.first_name}},\n\nDay 2 · resend with new subject.\n\n[Open your workspace](https://app.acme.example)\n\n---\n{{sender.postal_address}} · [Unsubscribe]({{links.unsubscribe}}) · [Preferences]({{links.preferences}})","body_format":"MARKDOWN","channel":"EMAIL","click_rate":0.06,"created_at":"2026-07-18T16:00:00Z","id":"40072573-6c82-5100-8709-5ed36d8c6794","is_archived":false,"kind":"AUTOMATION","language_code":"en","merge_fields":["contact.first_name","sender.postal_address","links.unsubscribe"],"name":"Day 2 · resend with new subject","open_rate":0.29,"preheader":"","priority":"NORMAL","sent_30d":602,"subject":"Still here? Three quick wins","unsub_30d":4,"updated_at":"2026-10-18T16:00:00Z","used_by":[{"step":"5 (no branch)","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"}],"version":1,"versions":[{"at":"2026-10-17T16:00:00Z","version":1}]})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 16, // TODO: Set from your business logic
    })
}
