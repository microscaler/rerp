// Implementation stub for handler 'get_deal_owner_overview'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::get_deal_owner_overview::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_insights_gen::handlers::types::DealOwnerRow;

#[handler(GetDealOwnerOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let pipeline_id = req.inner.pipeline_id;// let owner_id = req.inner.owner_id;// let from = req.inner.from;// let to = req.inner.to;// let currency = req.inner.currency;// let team_id = req.inner.team_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        currency: "EUR".to_string(), // TODO: Set from your business logic
        owners: vec![serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":29,"avg_deal_value":"8070.00","deals_created":31,"lost_count":5,"lost_value":"1250.00","name":"Alex Morgan","open_count":16,"open_value":"1250.00","tasks_completed":96,"user_id":"alex","win_rate":0.71,"won_count":12,"won_value":"96800.00"})).unwrap_or_default(), serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":38,"avg_deal_value":"7910.00","deals_created":27,"lost_count":8,"lost_value":"1250.00","name":"Sam Lee","open_count":10,"open_value":"1250.00","tasks_completed":64,"user_id":"sam","win_rate":0.53,"won_count":9,"won_value":"71200.00"})).unwrap_or_default(), serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":41,"avg_deal_value":"4600.00","deals_created":12,"lost_count":3,"lost_value":"1250.00","name":"Riya Kapoor","open_count":3,"open_value":"1250.00","tasks_completed":28,"user_id":"riya","win_rate":0.57,"won_count":4,"won_value":"18400.00"})).unwrap_or_default()], // TODO: Set from your business logic
        period: None, // TODO: Set from your business logic
    })
}
