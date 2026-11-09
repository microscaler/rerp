// User-owned controller for handler 'get_campaign'.

use crate::handlers::get_campaign::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ABTestConfig;
#[allow(unused_imports)]
use crate::handlers::types::CampaignAudience;
#[allow(unused_imports)]
use crate::handlers::types::CampaignKind;
#[allow(unused_imports)]
use crate::handlers::types::CampaignStatus;
#[allow(unused_imports)]
use crate::handlers::types::CampaignVariant;
#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::ReadinessProblem;
#[allow(unused_imports)]
use crate::handlers::types::ReminderConfig;
#[allow(unused_imports)]
use crate::handlers::types::TrackedLink;

#[handler(GetCampaignController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        ab_test: Some(Default::default()),
        audience: Some(Default::default()),
        audience_count: Some(42),
        channel: Default::default(),
        completed_at: Some("example".to_string()),
        created_at: "example".to_string(),
        created_by: Some("example".to_string()),
        exclude_list_ids: Some(vec![]),
        folder: Some("example".to_string()),
        id: "example".to_string(),
        kind: Default::default(),
        links: Some(vec![]),
        list_ids: vec![],
        name: "example".to_string(),
        on_open_workflow_ids: Some(vec![]),
        postal_address_id: Some("example".to_string()),
        preheader: Some("example".to_string()),
        public_archive: Some(true),
        readiness: Some(vec![]),
        recurrence: Some(Default::default()),
        reminder: Some(Default::default()),
        reply_to: Some("example".to_string()),
        segment: Some(Default::default()),
        send_at: Some("example".to_string()),
        send_in_contact_timezone: Some(true),
        sender_id: Some("example".to_string()),
        sent_count: Some(42),
        started_at: Some("example".to_string()),
        status: Default::default(),
        template_id: Some("example".to_string()),
        track_clicks: Some(true),
        track_opens: Some(true),
        track_replies: Some(true),
        updated_at: Some("example".to_string()),
        utm_campaign_id: Some("example".to_string()),
        variants: Some(vec![]),
    })
}
