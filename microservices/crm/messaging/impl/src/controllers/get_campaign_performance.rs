// Implementation stub for handler 'get_campaign_performance'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_messaging_gen::handlers::get_campaign_performance::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::CampaignPerformanceRowPage;
#[allow(unused_imports)]
use rerp_crm_messaging_gen::handlers::types::OutreachOverview;

#[handler(GetCampaignPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let kind = req.inner.kind;// let folder_id = req.inner.folder_id;// let list_id = req.inner.list_id;// let search = req.inner.search;// let from = req.inner.from;// let to = req.inner.to;// let compare_to = req.inner.compare_to;// let exclude_unverified_opens = req.inner.exclude_unverified_opens;// let sort = req.inner.sort;// let order = req.inner.order;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        campaigns: serde_json::from_value::<CampaignPerformanceRowPage>(serde_json::json!({"items":[{"campaign_id":"94cb11b7-4223-5b33-bb8d-320012c437a0","name":"Northwind Traders","sent":3}],"limit":25,"page":1,"total":2})).unwrap_or_default(), // TODO: Set from your business logic
        comparison: None, // TODO: Set from your business logic
        comparison_basis: None, // TODO: Set from your business logic
        totals: serde_json::from_value::<OutreachOverview>(serde_json::json!({"anonymous_open_rate":12.5,"bounce_rate":12.5,"campaigns_sent":3,"click_rate":12.5,"click_rate_delta_pts":12.5,"click_to_open_rate":12.5,"complaint_rate":12.5,"contact_growth":[{}],"contacts_added_30d":3,"messages_sent":3,"open_rate":12.5,"open_rate_delta_pts":12.5,"setup":[{}],"subscribed_contacts":3,"total_contacts":3,"unsubscribe_rate":12.5})).unwrap_or_default(), // TODO: Set from your business logic
    })
}
