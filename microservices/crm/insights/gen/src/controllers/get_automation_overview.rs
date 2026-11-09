// User-owned controller for handler 'get_automation_overview'.

use crate::handlers::get_automation_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AutomationMessageRow;
#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;

#[handler(GetAutomationOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "click_rate": 0.091,
    //   "completion_rate": 0.855,
    //   "contacts_finished": 1402,
    //   "contacts_total": 1640,
    //   "email_sent_rate": 0.992,
    //   "emails_sent": 6120,
    //   "messages": [
    //     {
    //       "click_rate": 0.129,
    //       "last_sent_at": "2026-11-05T16:00:00Z",
    //       "name": "Day 0 · Welcome",
    //       "open_rate": 0.66,
    //       "revenue": "0.00",
    //       "sends": 1640,
    //       "step_id": "23d73cc0-2fda-5946-a568-c94ed4a4b6fb",
    //       "unique_clicks": 211,
    //       "unique_opens": 1082
    //     },
    //     {
    //       "click_rate": 0.103,
    //       "last_sent_at": "2026-11-05T16:00:00Z",
    //       "name": "Day 1 · Getting started",
    //       "open_rate": 0.55,
    //       "revenue": "0.00",
    //       "sends": 1598,
    //       "step_id": "ef70b030-e13c-5bc8-8e8b-8ed1237f31ea",
    //       "unique_clicks": 164,
    //       "unique_opens": 879
    //     },
    //     {
    //       "click_rate": 0.087,
    //       "last_sent_at": "2026-11-05T16:00:00Z",
    //       "name": "Day 3 · Tips for power users",
    //       "open_rate": 0.5,
    //       "revenue": "1200.00",
    //       "sends": 1402,
    //       "step_id": "34840ea3-97fc-5c23-9cbc-94fdecf47898",
    //       "unique_clicks": 122,
    //       "unique_opens": 701
    //     }
    //   ],
    //   "name": "Welcome series (7 days)",
    //   "open_rate": 0.52,
    //   "revenue": "12400.00",
    //   "series": [
    //     {
    //       "key": "sends",
    //       "label": "Sends",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 204
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 224
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 242
    //         }
    //       ]
    //     },
    //     {
    //       "key": "opens",
    //       "label": "Opens",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 106
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 116
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 126
    //         }
    //       ]
    //     },
    //     {
    //       "key": "clicks",
    //       "label": "Clicks",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 18
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 20
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 21
    //         }
    //       ]
    //     }
    //   ],
    //   "unique_clicks": 557,
    //   "unique_opens": 3182,
    //   "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "click_rate": 0.091,
  "completion_rate": 0.855,
  "contacts_finished": 1402,
  "contacts_total": 1640,
  "email_sent_rate": 0.992,
  "emails_sent": 6120,
  "messages": [
    {
      "click_rate": 0.129,
      "last_sent_at": "2026-11-05T16:00:00Z",
      "name": "Day 0 · Welcome",
      "open_rate": 0.66,
      "revenue": "0.00",
      "sends": 1640,
      "step_id": "23d73cc0-2fda-5946-a568-c94ed4a4b6fb",
      "unique_clicks": 211,
      "unique_opens": 1082
    },
    {
      "click_rate": 0.103,
      "last_sent_at": "2026-11-05T16:00:00Z",
      "name": "Day 1 · Getting started",
      "open_rate": 0.55,
      "revenue": "0.00",
      "sends": 1598,
      "step_id": "ef70b030-e13c-5bc8-8e8b-8ed1237f31ea",
      "unique_clicks": 164,
      "unique_opens": 879
    },
    {
      "click_rate": 0.087,
      "last_sent_at": "2026-11-05T16:00:00Z",
      "name": "Day 3 · Tips for power users",
      "open_rate": 0.5,
      "revenue": "1200.00",
      "sends": 1402,
      "step_id": "34840ea3-97fc-5c23-9cbc-94fdecf47898",
      "unique_clicks": 122,
      "unique_opens": 701
    }
  ],
  "name": "Welcome series (7 days)",
  "open_rate": 0.52,
  "revenue": "12400.00",
  "series": [
    {
      "key": "sends",
      "label": "Sends",
      "points": [
        {
          "date": "2026-10-07",
          "value": 204
        },
        {
          "date": "2026-10-08",
          "value": 224
        },
        {
          "date": "2026-10-09",
          "value": 242
        }
      ]
    },
    {
      "key": "opens",
      "label": "Opens",
      "points": [
        {
          "date": "2026-10-07",
          "value": 106
        },
        {
          "date": "2026-10-08",
          "value": 116
        },
        {
          "date": "2026-10-09",
          "value": 126
        }
      ]
    },
    {
      "key": "clicks",
      "label": "Clicks",
      "points": [
        {
          "date": "2026-10-07",
          "value": 18
        },
        {
          "date": "2026-10-08",
          "value": 20
        },
        {
          "date": "2026-10-09",
          "value": 21
        }
      ]
    }
  ],
  "unique_clicks": 557,
  "unique_opens": 3182,
  "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        click_rate: Some(0.091),completion_rate: 0.855,contacts_finished: 1402,contacts_total: 1640,email_sent_rate: Some(0.992),emails_sent: 6120,messages: vec![serde_json::from_value::<AutomationMessageRow>(serde_json::json!({"click_rate":0.129,"last_sent_at":"2026-11-05T16:00:00Z","name":"Day 0 · Welcome","open_rate":0.66,"revenue":"0.00","sends":1640,"step_id":"23d73cc0-2fda-5946-a568-c94ed4a4b6fb","unique_clicks":211,"unique_opens":1082})).unwrap_or_default(), serde_json::from_value::<AutomationMessageRow>(serde_json::json!({"click_rate":0.103,"last_sent_at":"2026-11-05T16:00:00Z","name":"Day 1 · Getting started","open_rate":0.55,"revenue":"0.00","sends":1598,"step_id":"ef70b030-e13c-5bc8-8e8b-8ed1237f31ea","unique_clicks":164,"unique_opens":879})).unwrap_or_default(), serde_json::from_value::<AutomationMessageRow>(serde_json::json!({"click_rate":0.087,"last_sent_at":"2026-11-05T16:00:00Z","name":"Day 3 · Tips for power users","open_rate":0.5,"revenue":"1200.00","sends":1402,"step_id":"34840ea3-97fc-5c23-9cbc-94fdecf47898","unique_clicks":122,"unique_opens":701})).unwrap_or_default()],name: "Welcome series (7 days)".to_string(),open_rate: Some(0.52),revenue: Some("12400.00".to_string()),series: Some(vec![serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"sends","label":"Sends","points":[{"date":"2026-10-07","value":204},{"date":"2026-10-08","value":224},{"date":"2026-10-09","value":242}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"opens","label":"Opens","points":[{"date":"2026-10-07","value":106},{"date":"2026-10-08","value":116},{"date":"2026-10-09","value":126}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"clicks","label":"Clicks","points":[{"date":"2026-10-07","value":18},{"date":"2026-10-08","value":20},{"date":"2026-10-09","value":21}]})).unwrap_or_default()]),unique_clicks: Some(557),unique_opens: Some(3182),workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(),
    })
}
