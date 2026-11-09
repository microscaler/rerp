// User-owned controller for handler 'get_open_trends'.

use crate::handlers::get_open_trends::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetOpenTrendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "by_hour": [
    //     {
    //       "hour": 0,
    //       "opens": 200
    //     },
    //     {
    //       "hour": 1,
    //       "opens": 100
    //     },
    //     {
    //       "hour": 2,
    //       "opens": 100
    //     }
    //   ],
    //   "by_weekday": [
    //     {
    //       "opens": 6120,
    //       "weekday": 0
    //     },
    //     {
    //       "opens": 7804,
    //       "weekday": 1
    //     },
    //     {
    //       "opens": 7102,
    //       "weekday": 2
    //     }
    //   ],
    //   "timezone": "Europe/Sofia"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "by_hour": [
    {
      "hour": 0,
      "opens": 200
    },
    {
      "hour": 1,
      "opens": 100
    },
    {
      "hour": 2,
      "opens": 100
    }
  ],
  "by_weekday": [
    {
      "opens": 6120,
      "weekday": 0
    },
    {
      "opens": 7804,
      "weekday": 1
    },
    {
      "opens": 7102,
      "weekday": 2
    }
  ],
  "timezone": "Europe/Sofia"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        by_hour: vec![
            serde_json::from_str::<serde_json::Value>(r#"{"hour":0,"opens":200}"#)
                .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(r#"{"hour":1,"opens":100}"#)
                .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(r#"{"hour":2,"opens":100}"#)
                .unwrap_or(serde_json::Value::Null),
        ],
        by_weekday: vec![
            serde_json::from_str::<serde_json::Value>(r#"{"opens":6120,"weekday":0}"#)
                .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(r#"{"opens":7804,"weekday":1}"#)
                .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(r#"{"opens":7102,"weekday":2}"#)
                .unwrap_or(serde_json::Value::Null),
        ],
        timezone: Some("Europe/Sofia".to_string()),
    })
}
