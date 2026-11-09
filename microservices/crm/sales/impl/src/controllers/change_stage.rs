// Implementation stub for handler 'change_stage'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::change_stage::{Request, Response};

#[handler(ChangeStageController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let comment = req.inner.comment;// let lost_reason_id = req.inner.lost_reason_id;// let stage_id = req.inner.stage_id;// let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        active: true,                                    // TODO: Set from your business logic
        attribution: None,                               // TODO: Set from your business logic
        automated_probability: None,                     // TODO: Set from your business logic
        campaign_id: None,                               // TODO: Set from your business logic
        color: None,                                     // TODO: Set from your business logic
        contact_name: None,                              // TODO: Set from your business logic
        create_date: "2026-11-05T16:00:00Z".to_string(), // TODO: Set from your business logic
        date_closed: None,                               // TODO: Set from your business logic
        date_deadline: None,                             // TODO: Set from your business logic
        date_last_stage_update: None,                    // TODO: Set from your business logic
        date_open: None,                                 // TODO: Set from your business logic
        day_close: None,                                 // TODO: Set from your business logic
        day_open: None,                                  // TODO: Set from your business logic
        description: None,                               // TODO: Set from your business logic
        duplicate_lead_count: None,                      // TODO: Set from your business logic
        duplicate_lead_ids: None,                        // TODO: Set from your business logic
        email_from: None,                                // TODO: Set from your business logic
        email_normalized: None,                          // TODO: Set from your business logic
        expected_revenue: None,                          // TODO: Set from your business logic
        form: None,                                      // TODO: Set from your business logic
        function: None,                                  // TODO: Set from your business logic
        id: "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26".to_string(), // TODO: Set from your business logic
        is_automated_probability: None, // TODO: Set from your business logic
        is_blacklisted: None,           // TODO: Set from your business logic
        is_rotting: None,               // TODO: Set from your business logic
        lost_reason_id: None,           // TODO: Set from your business logic
        medium_id: None,                // TODO: Set from your business logic
        mobile: None,                   // TODO: Set from your business logic
        name: "Northwind Traders".to_string(), // TODO: Set from your business logic
        organisation_id: None,          // TODO: Set from your business logic
        organisation_name: None,        // TODO: Set from your business logic
        partner_id: None,               // TODO: Set from your business logic
        phone: None,                    // TODO: Set from your business logic
        phone_sanitized: None,          // TODO: Set from your business logic
        priority: None,                 // TODO: Set from your business logic
        probability: None,              // TODO: Set from your business logic
        prorated_revenue: None,         // TODO: Set from your business logic
        recurring_plan_id: None,        // TODO: Set from your business logic
        recurring_revenue: None,        // TODO: Set from your business logic
        recurring_revenue_monthly: None, // TODO: Set from your business logic
        referred_by: None,              // TODO: Set from your business logic
        source_id: None,                // TODO: Set from your business logic
        stage_color: None,              // TODO: Set from your business logic
        stage_id: None,                 // TODO: Set from your business logic
        stage_name: None,               // TODO: Set from your business logic
        stage_probability: None,        // TODO: Set from your business logic
        tag_ids: None,                  // TODO: Set from your business logic
        team_id: None,                  // TODO: Set from your business logic
        title: None,                    // TODO: Set from your business logic
        r#type: "LEAD".to_string(),     // TODO: Set from your business logic
        user_id: None,                  // TODO: Set from your business logic
        website: None,                  // TODO: Set from your business logic
        won_status: None,               // TODO: Set from your business logic
        write_date: None,               // TODO: Set from your business logic
        write_uid: None,                // TODO: Set from your business logic
    })
}
