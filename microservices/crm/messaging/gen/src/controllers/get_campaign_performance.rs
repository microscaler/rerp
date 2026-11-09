// User-owned controller for handler 'get_campaign_performance'.

use crate::handlers::get_campaign_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CampaignPerformanceRowPage;
#[allow(unused_imports)]
use crate::handlers::types::OutreachOverview;

#[handler(GetCampaignPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "campaigns": {
    //     "items": [
    //       {
    //         "campaign_id": "94cb11b7-4223-5b33-bb8d-320012c437a0",
    //         "name": "Northwind Traders",
    //         "sent": 3
    //       }
    //     ],
    //     "limit": 25,
    //     "page": 1,
    //     "total": 2
    //   },
    //   "comparison": {
    //     "anonymous_open_rate": 12.5,
    //     "bounce_rate": 12.5,
    //     "campaigns_sent": 3,
    //     "click_rate": 12.5,
    //     "click_rate_delta_pts": 12.5,
    //     "click_to_open_rate": 12.5,
    //     "complaint_rate": 12.5,
    //     "contact_growth": [
    //       {}
    //     ],
    //     "contacts_added_30d": 3,
    //     "messages_sent": 3,
    //     "open_rate": 12.5,
    //     "open_rate_delta_pts": 12.5,
    //     "setup": [
    //       {}
    //     ],
    //     "subscribed_contacts": 3,
    //     "total_contacts": 3,
    //     "unsubscribe_rate": 12.5
    //   },
    //   "comparison_basis": "NONE",
    //   "totals": {
    //     "anonymous_open_rate": 12.5,
    //     "bounce_rate": 12.5,
    //     "campaigns_sent": 3,
    //     "click_rate": 12.5,
    //     "click_rate_delta_pts": 12.5,
    //     "click_to_open_rate": 12.5,
    //     "complaint_rate": 12.5,
    //     "contact_growth": [
    //       {}
    //     ],
    //     "contacts_added_30d": 3,
    //     "messages_sent": 3,
    //     "open_rate": 12.5,
    //     "open_rate_delta_pts": 12.5,
    //     "setup": [
    //       {}
    //     ],
    //     "subscribed_contacts": 3,
    //     "total_contacts": 3,
    //     "unsubscribe_rate": 12.5
    //   }
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "campaigns": {
    "items": [
      {
        "campaign_id": "94cb11b7-4223-5b33-bb8d-320012c437a0",
        "name": "Northwind Traders",
        "sent": 3
      }
    ],
    "limit": 25,
    "page": 1,
    "total": 2
  },
  "comparison": {
    "anonymous_open_rate": 12.5,
    "bounce_rate": 12.5,
    "campaigns_sent": 3,
    "click_rate": 12.5,
    "click_rate_delta_pts": 12.5,
    "click_to_open_rate": 12.5,
    "complaint_rate": 12.5,
    "contact_growth": [
      {}
    ],
    "contacts_added_30d": 3,
    "messages_sent": 3,
    "open_rate": 12.5,
    "open_rate_delta_pts": 12.5,
    "setup": [
      {}
    ],
    "subscribed_contacts": 3,
    "total_contacts": 3,
    "unsubscribe_rate": 12.5
  },
  "comparison_basis": "NONE",
  "totals": {
    "anonymous_open_rate": 12.5,
    "bounce_rate": 12.5,
    "campaigns_sent": 3,
    "click_rate": 12.5,
    "click_rate_delta_pts": 12.5,
    "click_to_open_rate": 12.5,
    "complaint_rate": 12.5,
    "contact_growth": [
      {}
    ],
    "contacts_added_30d": 3,
    "messages_sent": 3,
    "open_rate": 12.5,
    "open_rate_delta_pts": 12.5,
    "setup": [
      {}
    ],
    "subscribed_contacts": 3,
    "total_contacts": 3,
    "unsubscribe_rate": 12.5
  }
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        campaigns: serde_json::from_value::<CampaignPerformanceRowPage>(serde_json::json!({"items":[{"campaign_id":"94cb11b7-4223-5b33-bb8d-320012c437a0","name":"Northwind Traders","sent":3}],"limit":25,"page":1,"total":2})).unwrap_or_default(),comparison: Some(serde_json::from_value::<OutreachOverview>(serde_json::json!({"anonymous_open_rate":12.5,"bounce_rate":12.5,"campaigns_sent":3,"click_rate":12.5,"click_rate_delta_pts":12.5,"click_to_open_rate":12.5,"complaint_rate":12.5,"contact_growth":[{}],"contacts_added_30d":3,"messages_sent":3,"open_rate":12.5,"open_rate_delta_pts":12.5,"setup":[{}],"subscribed_contacts":3,"total_contacts":3,"unsubscribe_rate":12.5})).unwrap_or_default()),comparison_basis: Some("NONE".to_string()),totals: serde_json::from_value::<OutreachOverview>(serde_json::json!({"anonymous_open_rate":12.5,"bounce_rate":12.5,"campaigns_sent":3,"click_rate":12.5,"click_rate_delta_pts":12.5,"click_to_open_rate":12.5,"complaint_rate":12.5,"contact_growth":[{}],"contacts_added_30d":3,"messages_sent":3,"open_rate":12.5,"open_rate_delta_pts":12.5,"setup":[{}],"subscribed_contacts":3,"total_contacts":3,"unsubscribe_rate":12.5})).unwrap_or_default(),
    })
}
