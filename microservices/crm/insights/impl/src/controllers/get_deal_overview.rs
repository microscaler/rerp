// Implementation stub for handler 'get_deal_overview'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::get_deal_overview::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::DealOwnerRow;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::DealStageRow;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::DealTotals;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::DealValueBracket;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::PeriodCount;

#[handler(GetDealOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let pipeline_id = req.inner.pipeline_id;// let owner_id = req.inner.owner_id;// let from = req.inner.from;// let to = req.inner.to;// let currency = req.inner.currency;// let status = req.inner.status;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        by_owner: vec![serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":29,"avg_deal_value":"8070.00","deals_created":31,"lost_count":5,"lost_value":"1250.00","name":"Alex Morgan","open_count":16,"open_value":"1250.00","tasks_completed":96,"user_id":"alex","win_rate":0.71,"won_count":12,"won_value":"96800.00"})).unwrap_or_default(), serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":38,"avg_deal_value":"7910.00","deals_created":27,"lost_count":8,"lost_value":"1250.00","name":"Sam Lee","open_count":10,"open_value":"1250.00","tasks_completed":64,"user_id":"sam","win_rate":0.53,"won_count":9,"won_value":"71200.00"})).unwrap_or_default(), serde_json::from_value::<DealOwnerRow>(serde_json::json!({"avg_days_to_close":41,"avg_deal_value":"4600.00","deals_created":12,"lost_count":3,"lost_value":"1250.00","name":"Riya Kapoor","open_count":3,"open_value":"1250.00","tasks_completed":28,"user_id":"riya","win_rate":0.57,"won_count":4,"won_value":"18400.00"})).unwrap_or_default()], // TODO: Set from your business logic
        by_stage: vec![serde_json::from_value::<DealStageRow>(serde_json::json!({"count":9,"name":"New","stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","value":"41000.00","weighted_value":"4100.00"})).unwrap_or_default(), serde_json::from_value::<DealStageRow>(serde_json::json!({"count":8,"name":"Qualified","stage_id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","value":"96400.00","weighted_value":"24100.00"})).unwrap_or_default(), serde_json::from_value::<DealStageRow>(serde_json::json!({"count":7,"name":"Proposal","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","value":"118500.00","weighted_value":"59250.00"})).unwrap_or_default()], // TODO: Set from your business logic
        created_per_period: None, // TODO: Set from your business logic
        currency: "EUR".to_string(), // TODO: Set from your business logic
        totals: serde_json::from_value::<DealTotals>(serde_json::json!({"avg_days_to_close":34,"avg_deal_value":"7456.00","lost_count":16,"lost_value":"98100.00","open_count":38,"open_value":"412300.00","win_rate":0.61,"won_count":25,"won_value":"186400.00"})).unwrap_or_default(), // TODO: Set from your business logic
        value_brackets: vec![serde_json::from_value::<DealValueBracket>(serde_json::json!({"count":11,"label":"< €1k","max":1000,"min":"0.00","value":"6200.00"})).unwrap_or_default(), serde_json::from_value::<DealValueBracket>(serde_json::json!({"count":20,"label":"€1k – €5k","max":5000,"min":"1000.00","value":"58400.00"})).unwrap_or_default(), serde_json::from_value::<DealValueBracket>(serde_json::json!({"count":14,"label":"€5k – €25k","max":25000,"min":"5000.00","value":"171600.00"})).unwrap_or_default()], // TODO: Set from your business logic
    })
}
