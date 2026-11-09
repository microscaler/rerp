// Implementation stub for handler 'list_saved_responses'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_saved_responses::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_sales_gen::handlers::types::SavedResponse;

#[handler(ListSavedResponsesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;// let category_id = req.inner.category_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<SavedResponse>(serde_json::json!({"body":"Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%","category":"Sales","category_id":"113ca5aa-e3f2-576a-b6b5-79dbb715308e","id":"9587a888-ee0e-50ac-8e1a-38a273fccd36","name":"Northwind Traders","subject":"How is your %FIRSTNAME% trial going?","title":"Trial follow-up","updated_at":"2026-11-04T16:00:00Z","use_count":41})).unwrap_or_default(), serde_json::from_value::<SavedResponse>(serde_json::json!({"body":"Hi %FIRSTNAME%,\n\nAttached is the proposal we discussed…\n\n%SIGNATURE%","category":"Sales","category_id":"113ca5aa-e3f2-576a-b6b5-79dbb715308e","id":"eac7c2fa-4409-5886-b674-5a0d0ed8ddb6","name":"Northwind Traders","subject":"Proposal for %ACCOUNT%","title":"Proposal cover note","updated_at":"2026-10-27T16:00:00Z","use_count":18})).unwrap_or_default(), serde_json::from_value::<SavedResponse>(serde_json::json!({"body":"Hi %FIRSTNAME%, see you on %MEETING_DATE%.\n\n%SIGNATURE%","category":"Scheduling","category_id":"cab98488-f673-5409-9af6-81de2f44a796","id":"5bad7783-ebef-5909-8e69-53319d7f4cfd","name":"Northwind Traders","subject":"Confirmed: %MEETING_DATE%","title":"Meeting confirmation","updated_at":"2026-10-16T16:00:00Z","use_count":66})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 4, // TODO: Set from your business logic
    })
}
