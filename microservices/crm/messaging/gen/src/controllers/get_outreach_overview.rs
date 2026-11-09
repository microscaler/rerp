// User-owned controller for handler 'get_outreach_overview'.

use crate::handlers::get_outreach_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetOutreachOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "bounce_rate": 0.009,
    //   "campaigns_sent": 6,
    //   "click_rate": 0.046,
    //   "click_rate_delta_pts": -0.4,
    //   "complaint_rate": 0.0002,
    //   "contact_growth": [
    //     {
    //       "month": "2025-12",
    //       "subscribed": 420,
    //       "unsubscribed": 40
    //     },
    //     {
    //       "month": "2026-01",
    //       "subscribed": 480,
    //       "unsubscribed": 50
    //     },
    //     {
    //       "month": "2026-02",
    //       "subscribed": 510,
    //       "unsubscribed": 40
    //     }
    //   ],
    //   "contacts_added_30d": 312,
    //   "messages_sent": 61402,
    //   "open_rate": 0.382,
    //   "open_rate_delta_pts": 2.1,
    //   "setup": [
    //     {
    //       "done": true,
    //       "key": "sender",
    //       "label": "Verify sending domain"
    //     },
    //     {
    //       "done": true,
    //       "key": "tracking",
    //       "label": "Install site tracking"
    //     },
    //     {
    //       "done": true,
    //       "key": "form",
    //       "label": "Publish a signup form"
    //     }
    //   ],
    //   "subscribed_contacts": 10912,
    //   "total_contacts": 12480,
    //   "unsubscribe_rate": 0.003
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "bounce_rate": 0.009,
  "campaigns_sent": 6,
  "click_rate": 0.046,
  "click_rate_delta_pts": -0.4,
  "complaint_rate": 0.0002,
  "contact_growth": [
    {
      "month": "2025-12",
      "subscribed": 420,
      "unsubscribed": 40
    },
    {
      "month": "2026-01",
      "subscribed": 480,
      "unsubscribed": 50
    },
    {
      "month": "2026-02",
      "subscribed": 510,
      "unsubscribed": 40
    }
  ],
  "contacts_added_30d": 312,
  "messages_sent": 61402,
  "open_rate": 0.382,
  "open_rate_delta_pts": 2.1,
  "setup": [
    {
      "done": true,
      "key": "sender",
      "label": "Verify sending domain"
    },
    {
      "done": true,
      "key": "tracking",
      "label": "Install site tracking"
    },
    {
      "done": true,
      "key": "form",
      "label": "Publish a signup form"
    }
  ],
  "subscribed_contacts": 10912,
  "total_contacts": 12480,
  "unsubscribe_rate": 0.003
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        anonymous_open_rate: Some(1.5),
        bounce_rate: Some(0.009),
        campaigns_sent: 6,
        click_rate: 0.046,
        click_rate_delta_pts: Some(-0.4),
        click_to_open_rate: Some(1.5),
        complaint_rate: Some(0.0002),
        contact_growth: Some(vec![
            serde_json::from_str::<serde_json::Value>(
                r#"{"month":"2025-12","subscribed":420,"unsubscribed":40}"#,
            )
            .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(
                r#"{"month":"2026-01","subscribed":480,"unsubscribed":50}"#,
            )
            .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(
                r#"{"month":"2026-02","subscribed":510,"unsubscribed":40}"#,
            )
            .unwrap_or(serde_json::Value::Null),
        ]),
        contacts_added_30d: Some(312),
        messages_sent: 61402,
        open_rate: 0.382,
        open_rate_delta_pts: Some(2.1),
        setup: Some(vec![
            serde_json::from_str::<serde_json::Value>(
                r#"{"done":true,"key":"sender","label":"Verify sending domain"}"#,
            )
            .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(
                r#"{"done":true,"key":"tracking","label":"Install site tracking"}"#,
            )
            .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(
                r#"{"done":true,"key":"form","label":"Publish a signup form"}"#,
            )
            .unwrap_or(serde_json::Value::Null),
        ]),
        subscribed_contacts: Some(10912),
        total_contacts: 12480,
        unsubscribe_rate: Some(0.003),
    })
}
