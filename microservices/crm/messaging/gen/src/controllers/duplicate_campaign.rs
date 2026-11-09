// User-owned controller for handler 'duplicate_campaign'.

use crate::handlers::duplicate_campaign::{Request, Response};
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
use crate::handlers::types::ConditionGroup;
#[allow(unused_imports)]
use crate::handlers::types::ReadinessProblem;
#[allow(unused_imports)]
use crate::handlers::types::ReminderConfig;
#[allow(unused_imports)]
use crate::handlers::types::TrackedLink;

#[handler(DuplicateCampaignController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "audience_count": 9870,
    //   "channel": "EMAIL",
    //   "completed_at": "2026-10-28T18:00:00Z",
    //   "created_at": "2026-10-25T16:00:00Z",
    //   "created_by": "alex",
    //   "exclude_list_ids": [],
    //   "folder": "Product",
    //   "id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
    //   "kind": "ONE_TIME",
    //   "list_ids": [
    //     "09f5b2f9-2adc-5edd-a6fd-c20675680671"
    //   ],
    //   "name": "October product update",
    //   "preheader": "",
    //   "segment": {
    //     "items": [
    //       {
    //         "operator": "EQ",
    //         "subject": "October product update"
    //       }
    //     ],
    //     "op": "AND"
    //   },
    //   "send_at": "2026-10-28T16:00:00Z",
    //   "send_in_contact_timezone": false,
    //   "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
    //   "sent_count": 9870,
    //   "started_at": "2026-10-28T16:00:00Z",
    //   "status": "SENT",
    //   "template_id": "2f1c88b7-46a3-5aad-8049-40ebd0b66079",
    //   "track_clicks": true,
    //   "track_opens": true,
    //   "track_replies": true,
    //   "updated_at": "2026-11-05T14:00:00Z",
    //   "utm_campaign_id": "d60cc555-1aa8-57d4-b917-88bbd0d5b437"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "audience_count": 9870,
  "channel": "EMAIL",
  "completed_at": "2026-10-28T18:00:00Z",
  "created_at": "2026-10-25T16:00:00Z",
  "created_by": "alex",
  "exclude_list_ids": [],
  "folder": "Product",
  "id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
  "kind": "ONE_TIME",
  "list_ids": [
    "09f5b2f9-2adc-5edd-a6fd-c20675680671"
  ],
  "name": "October product update",
  "preheader": "",
  "segment": {
    "items": [
      {
        "operator": "EQ",
        "subject": "October product update"
      }
    ],
    "op": "AND"
  },
  "send_at": "2026-10-28T16:00:00Z",
  "send_in_contact_timezone": false,
  "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
  "sent_count": 9870,
  "started_at": "2026-10-28T16:00:00Z",
  "status": "SENT",
  "template_id": "2f1c88b7-46a3-5aad-8049-40ebd0b66079",
  "track_clicks": true,
  "track_opens": true,
  "track_replies": true,
  "updated_at": "2026-11-05T14:00:00Z",
  "utm_campaign_id": "d60cc555-1aa8-57d4-b917-88bbd0d5b437"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        ab_test: Some(Default::default()),audience: Some(Default::default()),audience_count: Some(9870),channel: serde_json::from_value::<Channel>(serde_json::Value::String("EMAIL".to_string())).unwrap_or_default(),completed_at: Some("2026-10-28T18:00:00Z".to_string()),created_at: "2026-10-25T16:00:00Z".to_string(),created_by: Some("alex".to_string()),exclude_list_ids: Some(vec![]),folder: Some("Product".to_string()),id: "7de39bcc-4bc6-551b-a967-7b1629b846be".to_string(),kind: serde_json::from_value::<CampaignKind>(serde_json::Value::String("ONE_TIME".to_string())).unwrap_or_default(),links: Some(vec![]),list_ids: vec!["09f5b2f9-2adc-5edd-a6fd-c20675680671".to_string()],name: "October product update".to_string(),on_open_workflow_ids: Some(vec![]),postal_address_id: Some("example".to_string()),preheader: Some("".to_string()),public_archive: Some(true),readiness: Some(vec![]),recurrence: Some(Default::default()),reminder: Some(Default::default()),reply_to: Some("example".to_string()),segment: Some(serde_json::from_value::<ConditionGroup>(serde_json::json!({"items":[{"operator":"EQ","subject":"October product update"}],"op":"AND"})).unwrap_or_default()),send_at: Some("2026-10-28T16:00:00Z".to_string()),send_in_contact_timezone: Some(false),sender_id: Some("37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2".to_string()),sent_count: Some(9870),started_at: Some("2026-10-28T16:00:00Z".to_string()),status: serde_json::from_value::<CampaignStatus>(serde_json::Value::String("SENT".to_string())).unwrap_or_default(),template_id: Some("2f1c88b7-46a3-5aad-8049-40ebd0b66079".to_string()),track_clicks: Some(true),track_opens: Some(true),track_replies: Some(true),updated_at: Some("2026-11-05T14:00:00Z".to_string()),utm_campaign_id: Some("d60cc555-1aa8-57d4-b917-88bbd0d5b437".to_string()),variants: Some(vec![]),
    })
}
