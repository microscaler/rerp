// User-owned controller for handler 'list_stages'.

use crate::handlers::list_stages::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Stage;

#[handler(ListStagesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "card_fields": [
    //         "title",
    //         "next_task",
    //         "owner"
    //       ],
    //       "color": "#C9CFD8",
    //       "fold": false,
    //       "id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
    //       "is_lost": false,
    //       "is_won": false,
    //       "name": "New",
    //       "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //       "probability": 10,
    //       "rotting_days": 14,
    //       "sequence": 1,
    //       "sort_by": "NEXT_TASK",
    //       "team_ids": [
    //         "00000000-0000-0000-0000-000000000401"
    //       ]
    //     },
    //     {
    //       "card_fields": [
    //         "title",
    //         "next_task",
    //         "owner"
    //       ],
    //       "color": "#9DB6F7",
    //       "fold": false,
    //       "id": "8d4abc9c-648c-5dbe-8668-b73ce7d9a71e",
    //       "is_lost": false,
    //       "is_won": false,
    //       "name": "Qualified",
    //       "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //       "probability": 25,
    //       "rotting_days": 14,
    //       "sequence": 2,
    //       "sort_by": "NEXT_TASK",
    //       "team_ids": [
    //         "00000000-0000-0000-0000-000000000401"
    //       ]
    //     },
    //     {
    //       "card_fields": [
    //         "title",
    //         "next_task",
    //         "owner"
    //       ],
    //       "color": "#1F5AF6",
    //       "fold": false,
    //       "id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
    //       "is_lost": false,
    //       "is_won": false,
    //       "name": "Proposal",
    //       "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //       "probability": 50,
    //       "rotting_days": 14,
    //       "sequence": 3,
    //       "sort_by": "NEXT_TASK",
    //       "team_ids": [
    //         "00000000-0000-0000-0000-000000000401"
    //       ]
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 16
    // }
    match serde_json::from_str::<Response>(
        r##"{
  "items": [
    {
      "card_fields": [
        "title",
        "next_task",
        "owner"
      ],
      "color": "#C9CFD8",
      "fold": false,
      "id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
      "is_lost": false,
      "is_won": false,
      "name": "New",
      "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
      "probability": 10,
      "rotting_days": 14,
      "sequence": 1,
      "sort_by": "NEXT_TASK",
      "team_ids": [
        "00000000-0000-0000-0000-000000000401"
      ]
    },
    {
      "card_fields": [
        "title",
        "next_task",
        "owner"
      ],
      "color": "#9DB6F7",
      "fold": false,
      "id": "8d4abc9c-648c-5dbe-8668-b73ce7d9a71e",
      "is_lost": false,
      "is_won": false,
      "name": "Qualified",
      "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
      "probability": 25,
      "rotting_days": 14,
      "sequence": 2,
      "sort_by": "NEXT_TASK",
      "team_ids": [
        "00000000-0000-0000-0000-000000000401"
      ]
    },
    {
      "card_fields": [
        "title",
        "next_task",
        "owner"
      ],
      "color": "#1F5AF6",
      "fold": false,
      "id": "0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5",
      "is_lost": false,
      "is_won": false,
      "name": "Proposal",
      "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
      "probability": 50,
      "rotting_days": 14,
      "sequence": 3,
      "sort_by": "NEXT_TASK",
      "team_ids": [
        "00000000-0000-0000-0000-000000000401"
      ]
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 16
}"##,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Stage>(serde_json::json!({"card_fields":["title","next_task","owner"],"color":"#C9CFD8","fold":false,"id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","is_lost":false,"is_won":false,"name":"New","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":10,"rotting_days":14,"sequence":1,"sort_by":"NEXT_TASK","team_ids":["00000000-0000-0000-0000-000000000401"]})).unwrap_or_default(), serde_json::from_value::<Stage>(serde_json::json!({"card_fields":["title","next_task","owner"],"color":"#9DB6F7","fold":false,"id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","is_lost":false,"is_won":false,"name":"Qualified","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":25,"rotting_days":14,"sequence":2,"sort_by":"NEXT_TASK","team_ids":["00000000-0000-0000-0000-000000000401"]})).unwrap_or_default(), serde_json::from_value::<Stage>(serde_json::json!({"card_fields":["title","next_task","owner"],"color":"#1F5AF6","fold":false,"id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","is_lost":false,"is_won":false,"name":"Proposal","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":50,"rotting_days":14,"sequence":3,"sort_by":"NEXT_TASK","team_ids":["00000000-0000-0000-0000-000000000401"]})).unwrap_or_default()],limit: 25,page: 1,total: 16,
    })
}
