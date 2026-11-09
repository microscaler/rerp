// Implementation stub for handler 'list_scores'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::list_scores::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_insights_gen::handlers::types::Score;

#[handler(ListScoresController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let entity = req.inner.entity;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Score>(serde_json::json!({"counts":{"cold":8920,"hot":916,"warm":2644},"created_at":"2026-07-28T16:00:00Z","description":"email + site + lifecycle","distribution":[42,30,22],"entity":"CONTACT","id":"cef670be-5b12-52e7-8232-19a36fb64702","name":"Engagement","rules":[{"decay_days":30,"hits_30d":23410,"id":"5876ef17-a412-5e28-bf53-4b86e2f712d0","label":"Opened any email","operator":"HAS","points":2,"subject":"engagement:opened:30","value":""},{"decay_days":30,"hits_30d":2804,"id":"e5c78629-9b55-50f5-b029-293852d13941","label":"Clicked any email","operator":"HAS","points":5,"subject":"engagement:clicked:30","value":""},{"decay_days":14,"hits_30d":4019,"id":"45d0c445-7fa8-5368-8120-32d4993f83b5","label":"Visited /pricing","operator":"HAS","points":15,"subject":"site:visited:/pricing","value":""}],"status":"ACTIVE","thresholds":{"hot":70,"warm":40}})).unwrap_or_default(), serde_json::from_value::<Score>(serde_json::json!({"counts":{"cold":8920,"hot":916,"warm":2644},"created_at":"2026-07-28T16:00:00Z","description":"pricing and docs","distribution":[42,30,22],"entity":"CONTACT","id":"530ab6c1-e050-50d5-b714-96ac60fa8b94","name":"Website intent","rules":[{"decay_days":14,"hits_30d":4019,"id":"df5dd74d-8987-55cd-8f0a-9e56429e57c4","label":"Visited /pricing","operator":"HAS","points":10,"subject":"site:visited:/pricing","value":""},{"decay_days":14,"hits_30d":7118,"id":"4bf6fdd9-1e85-5f56-b83a-ce6f7049d29e","label":"Visited docs","operator":"CONTAINS","points":3,"subject":"site:visited:/docs/","value":""}],"status":"ACTIVE","thresholds":{"hot":70,"warm":40}})).unwrap_or_default(), serde_json::from_value::<Score>(serde_json::json!({"counts":{"cold":8920,"hot":916,"warm":2644},"created_at":"2026-07-28T16:00:00Z","description":"stage and activity","distribution":[42,30,22],"entity":"DEAL","id":"84cd7599-be34-5553-9d3c-e5e16028b7da","name":"Deal health","rules":[{"decay_days":0,"hits_30d":7,"id":"ca5ada16-e031-5ad5-81c1-599ebc84fa1c","label":"Stage Proposal","operator":"EQ","points":20,"subject":"deal:stage","value":"Proposal"},{"decay_days":0,"hits_30d":5,"id":"a98a2826-9aea-5c82-94e5-41e4aeb3a94f","label":"Stage Negotiation","operator":"EQ","points":40,"subject":"deal:stage","value":"Negotiation"},{"decay_days":0,"hits_30d":6,"id":"39fbcb83-e56a-5a92-86df-e2f2ec9c6118","label":"No activity 14 d","operator":"NOT_WITHIN_DAYS","points":-15,"subject":"last_activity_at","value":"14"}],"status":"ACTIVE","thresholds":{"hot":70,"warm":40}})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 4, // TODO: Set from your business logic
    })
}
