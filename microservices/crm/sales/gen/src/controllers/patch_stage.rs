// User-owned controller for handler 'patch_stage'.

use crate::handlers::patch_stage::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PatchStageController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "card_fields": [
    //     "title",
    //     "next_task",
    //     "owner"
    //   ],
    //   "color": "#C9CFD8",
    //   "fold": false,
    //   "id": "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e",
    //   "is_lost": false,
    //   "is_won": false,
    //   "name": "New",
    //   "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //   "probability": 10,
    //   "rotting_days": 14,
    //   "sequence": 1,
    //   "sort_by": "NEXT_TASK",
    //   "team_ids": [
    //     "00000000-0000-0000-0000-000000000401"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r##"{
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
}"##,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        card_fields: Some(vec![
            "title".to_string(),
            "next_task".to_string(),
            "owner".to_string(),
        ]),
        color: Some("#C9CFD8".to_string()),
        deal_order: Some("example".to_string()),
        description: Some("example".to_string()),
        fold: Some(false),
        id: "69eb8e38-4dda-5cd4-a12a-5ceefe3e043e".to_string(),
        is_lost: Some(false),
        is_won: Some(false),
        lead_count: Some(42),
        name: "New".to_string(),
        pipeline_id: Some("d9a2022e-96ff-53a3-a94d-2b169424fd31".to_string()),
        probability: 10,
        requirements: Some("example".to_string()),
        rotting_days: Some(14),
        rotting_threshold_days: Some(42),
        sequence: 1,
        sort_by: Some("NEXT_TASK".to_string()),
        team_ids: Some(vec!["00000000-0000-0000-0000-000000000401".to_string()]),
    })
}
