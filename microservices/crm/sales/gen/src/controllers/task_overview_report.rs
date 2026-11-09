// User-owned controller for handler 'task_overview_report'.

use crate::handlers::task_overview_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TaskOverviewReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "by_outcome": {
    //     "key": 3
    //   },
    //   "by_owner": [
    //     {}
    //   ],
    //   "by_sentiment": {
    //     "key": 3
    //   },
    //   "by_status": {
    //     "key": 3
    //   },
    //   "by_type": {
    //     "key": 3
    //   },
    //   "completed": 3,
    //   "open": 3,
    //   "overdue": 3,
    //   "series": [
    //     {}
    //   ],
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "by_outcome": {
    "key": 3
  },
  "by_owner": [
    {}
  ],
  "by_sentiment": {
    "key": 3
  },
  "by_status": {
    "key": 3
  },
  "by_type": {
    "key": 3
  },
  "completed": 3,
  "open": 3,
  "overdue": 3,
  "series": [
    {}
  ],
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        by_outcome: Some(
            serde_json::from_str::<serde_json::Value>(r#"{"key":3}"#)
                .unwrap_or(serde_json::Value::Null),
        ),
        by_owner: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ]),
        by_sentiment: Some(
            serde_json::from_str::<serde_json::Value>(r#"{"key":3}"#)
                .unwrap_or(serde_json::Value::Null),
        ),
        by_status: Some(
            serde_json::from_str::<serde_json::Value>(r#"{"key":3}"#)
                .unwrap_or(serde_json::Value::Null),
        ),
        by_type: Some(
            serde_json::from_str::<serde_json::Value>(r#"{"key":3}"#)
                .unwrap_or(serde_json::Value::Null),
        ),
        completed: Some(3),
        open: Some(3),
        overdue: Some(3),
        series: Some(vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ]),
        total: Some(2),
    })
}
