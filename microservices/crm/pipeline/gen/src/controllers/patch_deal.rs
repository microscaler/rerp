// User-owned controller for handler 'patch_deal'.

use crate::handlers::patch_deal::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealContact;
#[allow(unused_imports)]
use crate::handlers::types::DealStatus;

#[handler(PatchDealController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: Some("example".to_string()),
        company_name: Some("example".to_string()),
        contacts: Some(vec![]),
        created_at: "example".to_string(),
        created_by: Some("example".to_string()),
        currency: "example".to_string(),
        custom_fields: Some(Default::default()),
        days_in_stage: Some(42),
        description: Some("example".to_string()),
        expected_close_at: Some("example".to_string()),
        id: "example".to_string(),
        last_activity_at: Some("example".to_string()),
        lost_reason: Some("example".to_string()),
        next_task_at: Some("example".to_string()),
        open_task_count: Some(42),
        owner_id: Some("example".to_string()),
        owner_name: Some("example".to_string()),
        pipeline_id: "example".to_string(),
        pipeline_name: Some("example".to_string()),
        primary_contact_id: Some("example".to_string()),
        probability: Some(42),
        source: Some("example".to_string()),
        stage_entered_at: Some("example".to_string()),
        stage_id: "example".to_string(),
        stage_name: Some("example".to_string()),
        status: Default::default(),
        status_changed_at: Some("example".to_string()),
        tag_ids: Some(vec![]),
        title: "example".to_string(),
        updated_at: Some("example".to_string()),
        value: 1.5,
        weighted_value: Some(1.5),
    })
}
