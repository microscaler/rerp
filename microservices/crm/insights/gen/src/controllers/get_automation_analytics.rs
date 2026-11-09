// User-owned controller for handler 'get_automation_analytics'.

use crate::handlers::get_automation_analytics::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAutomationAnalyticsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "completed_30d": 1402,
    //       "deals_created": 0,
    //       "entered_30d": 1640,
    //       "goal_rate": 0.23,
    //       "messages_30d": 6120,
    //       "name": "Welcome series (7 days)",
    //       "open_rate": 0.52,
    //       "revenue": "0.00",
    //       "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    //     },
    //     {
    //       "completed_30d": 611,
    //       "deals_created": 0,
    //       "entered_30d": 902,
    //       "goal_rate": 0.11,
    //       "messages_30d": 2406,
    //       "name": "Abandoned signup",
    //       "open_rate": 0.34,
    //       "revenue": "0.00",
    //       "workflow_id": "1114e157-f1ec-5dda-9063-79abf0255ee3"
    //     },
    //     {
    //       "completed_30d": 1800,
    //       "deals_created": 0,
    //       "entered_30d": 2077,
    //       "goal_rate": 0.06,
    //       "messages_30d": 2077,
    //       "name": "Re-engage · 90 days quiet",
    //       "open_rate": 0.21,
    //       "revenue": "0.00",
    //       "workflow_id": "b36c0bc7-7dae-56b4-af7f-9c1ef4673496"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "completed_30d": 1402,
      "deals_created": 0,
      "entered_30d": 1640,
      "goal_rate": 0.23,
      "messages_30d": 6120,
      "name": "Welcome series (7 days)",
      "open_rate": 0.52,
      "revenue": "0.00",
      "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    },
    {
      "completed_30d": 611,
      "deals_created": 0,
      "entered_30d": 902,
      "goal_rate": 0.11,
      "messages_30d": 2406,
      "name": "Abandoned signup",
      "open_rate": 0.34,
      "revenue": "0.00",
      "workflow_id": "1114e157-f1ec-5dda-9063-79abf0255ee3"
    },
    {
      "completed_30d": 1800,
      "deals_created": 0,
      "entered_30d": 2077,
      "goal_rate": 0.06,
      "messages_30d": 2077,
      "name": "Re-engage · 90 days quiet",
      "open_rate": 0.21,
      "revenue": "0.00",
      "workflow_id": "b36c0bc7-7dae-56b4-af7f-9c1ef4673496"
    }
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response(vec![serde_json::from_str::<serde_json::Value>(r#"{"completed_30d":1402,"deals_created":0,"entered_30d":1640,"goal_rate":0.23,"messages_30d":6120,"name":"Welcome series (7 days)","open_rate":0.52,"revenue":"0.00","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"completed_30d":611,"deals_created":0,"entered_30d":902,"goal_rate":0.11,"messages_30d":2406,"name":"Abandoned signup","open_rate":0.34,"revenue":"0.00","workflow_id":"1114e157-f1ec-5dda-9063-79abf0255ee3"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"completed_30d":1800,"deals_created":0,"entered_30d":2077,"goal_rate":0.06,"messages_30d":2077,"name":"Re-engage · 90 days quiet","open_rate":0.21,"revenue":"0.00","workflow_id":"b36c0bc7-7dae-56b4-af7f-9c1ef4673496"}"#).unwrap_or(serde_json::Value::Null)]))
}
