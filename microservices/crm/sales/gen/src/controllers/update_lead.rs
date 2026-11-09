// User-owned controller for handler 'update_lead'.

use crate::handlers::update_lead::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateLeadController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "active": true,
    //   "contact_name": "Priya Natarajan",
    //   "create_date": "2026-10-13T16:00:00Z",
    //   "date_last_stage_update": "2026-11-02T16:00:00Z",
    //   "description": "Source: customer expansion",
    //   "email_from": "priya.natarajan@northwindtraders.example",
    //   "expected_revenue": "38400.00",
    //   "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //   "name": "Northwind · 48 → 120 seats",
    //   "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //   "organisation_name": "Northwind Traders",
    //   "phone": "+49 30 1000000",
    //   "priority": "NORMAL",
    //   "probability": 50,
    //   "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //   "stage_name": "Proposal",
    //   "stage_probability": 50,
    //   "tag_ids": [],
    //   "team_id": "00000000-0000-0000-0000-000000000401",
    //   "type": "LEAD",
    //   "won_status": "PENDING",
    //   "write_date": "2026-11-02T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "active": true,
  "contact_name": "Priya Natarajan",
  "create_date": "2026-10-13T16:00:00Z",
  "date_last_stage_update": "2026-11-02T16:00:00Z",
  "description": "Source: customer expansion",
  "email_from": "priya.natarajan@northwindtraders.example",
  "expected_revenue": "38400.00",
  "id": "63a6661f-5860-50ed-8697-f28d29a3917f",
  "name": "Northwind · 48 → 120 seats",
  "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
  "organisation_name": "Northwind Traders",
  "phone": "+49 30 1000000",
  "priority": "NORMAL",
  "probability": 50,
  "stage_id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
  "stage_name": "Proposal",
  "stage_probability": 50,
  "tag_ids": [],
  "team_id": "00000000-0000-0000-0000-000000000401",
  "type": "LEAD",
  "won_status": "PENDING",
  "write_date": "2026-11-02T16:00:00Z"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        active: true,
        automated_probability: Some(1.5),
        campaign_id: Some("example".to_string()),
        color: Some(42),
        contact_name: Some("Priya Natarajan".to_string()),
        create_date: "2026-10-13T16:00:00Z".to_string(),
        date_closed: Some("example".to_string()),
        date_deadline: Some("example".to_string()),
        date_last_stage_update: Some("2026-11-02T16:00:00Z".to_string()),
        date_open: Some("example".to_string()),
        day_close: Some(1.5),
        day_open: Some(1.5),
        description: Some("Source: customer expansion".to_string()),
        duplicate_lead_count: Some(42),
        duplicate_lead_ids: Some(vec![]),
        email_from: Some("priya.natarajan@northwindtraders.example".to_string()),
        email_normalized: Some("example".to_string()),
        expected_revenue: Some("38400.00".to_string()),
        function: Some("example".to_string()),
        id: "63a6661f-5860-50ed-8697-f28d29a3917f".to_string(),
        is_automated_probability: Some(true),
        is_blacklisted: Some(true),
        is_rotting: Some(true),
        lost_reason_id: Some("example".to_string()),
        medium_id: Some("example".to_string()),
        mobile: Some("example".to_string()),
        name: "Northwind · 48 → 120 seats".to_string(),
        organisation_id: Some("d7a2e2b8-61b1-5687-8aee-0ead8943ba3b".to_string()),
        organisation_name: Some("Northwind Traders".to_string()),
        partner_id: Some("example".to_string()),
        phone: Some("+49 30 1000000".to_string()),
        phone_sanitized: Some("example".to_string()),
        priority: Some("NORMAL".to_string()),
        probability: Some(50.0),
        prorated_revenue: Some("example".to_string()),
        recurring_plan_id: Some("example".to_string()),
        recurring_revenue: Some("example".to_string()),
        recurring_revenue_monthly: Some("example".to_string()),
        referred_by: Some("example".to_string()),
        source_id: Some("example".to_string()),
        stage_color: Some(42),
        stage_id: Some("0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5".to_string()),
        stage_name: Some("Proposal".to_string()),
        stage_probability: Some(50),
        tag_ids: Some(vec![]),
        team_id: Some("00000000-0000-0000-0000-000000000401".to_string()),
        title: Some("example".to_string()),
        r#type: "LEAD".to_string(),
        user_id: Some("example".to_string()),
        website: Some("example".to_string()),
        won_status: Some("PENDING".to_string()),
        write_date: Some("2026-11-02T16:00:00Z".to_string()),
        write_uid: Some("example".to_string()),
    })
}
