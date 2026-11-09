// User-owned controller for handler 'get_task_overview'.

use crate::handlers::get_task_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PeriodCount;
#[allow(unused_imports)]
use crate::handlers::types::TaskOutcomeRow;
#[allow(unused_imports)]
use crate::handlers::types::TaskOwnerRow;
#[allow(unused_imports)]
use crate::handlers::types::TaskTotals;
#[allow(unused_imports)]
use crate::handlers::types::TaskTypeRow;

#[handler(GetTaskOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "by_outcome": [
    //     {
    //       "count": 44,
    //       "label": "Reached",
    //       "outcome_id": "0a7437f6-0e3e-51ae-a9fd-4a44b900f91e",
    //       "sentiment": "POSITIVE"
    //     },
    //     {
    //       "count": 23,
    //       "label": "Replied",
    //       "outcome_id": "ca29541d-dde8-5a9e-818d-b0717df850c9",
    //       "sentiment": "POSITIVE"
    //     },
    //     {
    //       "count": 25,
    //       "label": "Held",
    //       "outcome_id": "822890d5-c8b4-5558-a84c-60e98767732f",
    //       "sentiment": "POSITIVE"
    //     }
    //   ],
    //   "by_owner": [
    //     {
    //       "completed": 96,
    //       "completion_rate": 0.84,
    //       "name": "Alex Morgan",
    //       "open": 18,
    //       "outcomes": {
    //         "negative": 11,
    //         "neutral": 24,
    //         "positive": 61
    //       },
    //       "overdue": 2,
    //       "user_id": "alex"
    //     },
    //     {
    //       "completed": 64,
    //       "completion_rate": 0.8,
    //       "name": "Sam Lee",
    //       "open": 16,
    //       "outcomes": {
    //         "negative": 12,
    //         "neutral": 22,
    //         "positive": 30
    //       },
    //       "overdue": 0,
    //       "user_id": "sam"
    //     },
    //     {
    //       "completed": 28,
    //       "completion_rate": 0.78,
    //       "name": "Riya Kapoor",
    //       "open": 8,
    //       "outcomes": {
    //         "negative": 4,
    //         "neutral": 9,
    //         "positive": 15
    //       },
    //       "overdue": 4,
    //       "user_id": "riya"
    //     }
    //   ],
    //   "by_type": [
    //     {
    //       "completed": 84,
    //       "completion_rate": 0.82,
    //       "name": "Call",
    //       "open": 18,
    //       "overdue": 2,
    //       "task_type_id": "4eec3526-5869-5e76-820f-ac2358c6b899"
    //     },
    //     {
    //       "completed": 61,
    //       "completion_rate": 0.85,
    //       "name": "Email",
    //       "open": 12,
    //       "overdue": 1,
    //       "task_type_id": "099104fc-dcee-57bf-b906-d2b826beb51d"
    //     },
    //     {
    //       "completed": 29,
    //       "completion_rate": 0.81,
    //       "name": "Meeting",
    //       "open": 7,
    //       "overdue": 2,
    //       "task_type_id": "645bfe17-69c3-558e-82cd-9594c3609fa2"
    //     }
    //   ],
    //   "totals": {
    //     "avg_hours_to_complete": 31,
    //     "completed": 188,
    //     "completion_rate": 0.82,
    //     "due_today": 4,
    //     "open": 42,
    //     "overdue": 6
    //   }
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "by_outcome": [
    {
      "count": 44,
      "label": "Reached",
      "outcome_id": "0a7437f6-0e3e-51ae-a9fd-4a44b900f91e",
      "sentiment": "POSITIVE"
    },
    {
      "count": 23,
      "label": "Replied",
      "outcome_id": "ca29541d-dde8-5a9e-818d-b0717df850c9",
      "sentiment": "POSITIVE"
    },
    {
      "count": 25,
      "label": "Held",
      "outcome_id": "822890d5-c8b4-5558-a84c-60e98767732f",
      "sentiment": "POSITIVE"
    }
  ],
  "by_owner": [
    {
      "completed": 96,
      "completion_rate": 0.84,
      "name": "Alex Morgan",
      "open": 18,
      "outcomes": {
        "negative": 11,
        "neutral": 24,
        "positive": 61
      },
      "overdue": 2,
      "user_id": "alex"
    },
    {
      "completed": 64,
      "completion_rate": 0.8,
      "name": "Sam Lee",
      "open": 16,
      "outcomes": {
        "negative": 12,
        "neutral": 22,
        "positive": 30
      },
      "overdue": 0,
      "user_id": "sam"
    },
    {
      "completed": 28,
      "completion_rate": 0.78,
      "name": "Riya Kapoor",
      "open": 8,
      "outcomes": {
        "negative": 4,
        "neutral": 9,
        "positive": 15
      },
      "overdue": 4,
      "user_id": "riya"
    }
  ],
  "by_type": [
    {
      "completed": 84,
      "completion_rate": 0.82,
      "name": "Call",
      "open": 18,
      "overdue": 2,
      "task_type_id": "4eec3526-5869-5e76-820f-ac2358c6b899"
    },
    {
      "completed": 61,
      "completion_rate": 0.85,
      "name": "Email",
      "open": 12,
      "overdue": 1,
      "task_type_id": "099104fc-dcee-57bf-b906-d2b826beb51d"
    },
    {
      "completed": 29,
      "completion_rate": 0.81,
      "name": "Meeting",
      "open": 7,
      "overdue": 2,
      "task_type_id": "645bfe17-69c3-558e-82cd-9594c3609fa2"
    }
  ],
  "totals": {
    "avg_hours_to_complete": 31,
    "completed": 188,
    "completion_rate": 0.82,
    "due_today": 4,
    "open": 42,
    "overdue": 6
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
        by_outcome: vec![serde_json::from_value::<TaskOutcomeRow>(serde_json::json!({"count":44,"label":"Reached","outcome_id":"0a7437f6-0e3e-51ae-a9fd-4a44b900f91e","sentiment":"POSITIVE"})).unwrap_or_default(), serde_json::from_value::<TaskOutcomeRow>(serde_json::json!({"count":23,"label":"Replied","outcome_id":"ca29541d-dde8-5a9e-818d-b0717df850c9","sentiment":"POSITIVE"})).unwrap_or_default(), serde_json::from_value::<TaskOutcomeRow>(serde_json::json!({"count":25,"label":"Held","outcome_id":"822890d5-c8b4-5558-a84c-60e98767732f","sentiment":"POSITIVE"})).unwrap_or_default()],by_owner: vec![serde_json::from_value::<TaskOwnerRow>(serde_json::json!({"completed":96,"completion_rate":0.84,"name":"Alex Morgan","open":18,"outcomes":{"negative":11,"neutral":24,"positive":61},"overdue":2,"user_id":"alex"})).unwrap_or_default(), serde_json::from_value::<TaskOwnerRow>(serde_json::json!({"completed":64,"completion_rate":0.8,"name":"Sam Lee","open":16,"outcomes":{"negative":12,"neutral":22,"positive":30},"overdue":0,"user_id":"sam"})).unwrap_or_default(), serde_json::from_value::<TaskOwnerRow>(serde_json::json!({"completed":28,"completion_rate":0.78,"name":"Riya Kapoor","open":8,"outcomes":{"negative":4,"neutral":9,"positive":15},"overdue":4,"user_id":"riya"})).unwrap_or_default()],by_type: vec![serde_json::from_value::<TaskTypeRow>(serde_json::json!({"completed":84,"completion_rate":0.82,"name":"Call","open":18,"overdue":2,"task_type_id":"4eec3526-5869-5e76-820f-ac2358c6b899"})).unwrap_or_default(), serde_json::from_value::<TaskTypeRow>(serde_json::json!({"completed":61,"completion_rate":0.85,"name":"Email","open":12,"overdue":1,"task_type_id":"099104fc-dcee-57bf-b906-d2b826beb51d"})).unwrap_or_default(), serde_json::from_value::<TaskTypeRow>(serde_json::json!({"completed":29,"completion_rate":0.81,"name":"Meeting","open":7,"overdue":2,"task_type_id":"645bfe17-69c3-558e-82cd-9594c3609fa2"})).unwrap_or_default()],completed_per_period: Some(vec![]),totals: serde_json::from_value::<TaskTotals>(serde_json::json!({"avg_hours_to_complete":31,"completed":188,"completion_rate":0.82,"due_today":4,"open":42,"overdue":6})).unwrap_or_default(),
    })
}
