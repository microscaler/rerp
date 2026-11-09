// User-owned controller for handler 'list_workflows'.

use crate::handlers::list_workflows::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Workflow;

#[handler(ListWorkflowsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "active_enrolments": 1204,
    //       "completed_enrolments": 8311,
    //       "created_at": "2026-06-08T16:00:00Z",
    //       "description": "",
    //       "entry_step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
    //       "id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
    //       "is_active": true,
    //       "last_run": "2026-11-05T15:58:00Z",
    //       "name": "Welcome series (7 days)",
    //       "reentry": "ONCE",
    //       "sequence": 1,
    //       "status": "ACTIVE",
    //       "trigger_type": "SUBSCRIBED_TO_LIST",
    //       "triggers": [
    //         {
    //           "config": {
    //             "label": "Newsletter"
    //           },
    //           "type": "SUBSCRIBED_TO_LIST"
    //         },
    //         {
    //           "config": {
    //             "label": "Website footer"
    //           },
    //           "type": "FORM_SUBMITTED"
    //         }
    //       ],
    //       "updated_at": "2026-11-02T16:00:00Z"
    //     },
    //     {
    //       "active_enrolments": 318,
    //       "completed_enrolments": 2940,
    //       "created_at": "2026-06-18T16:00:00Z",
    //       "description": "",
    //       "entry_step_id": "686016a9-b02e-5415-8f48-28549c0ca454",
    //       "id": "1114e157-f1ec-5dda-9063-79abf0255ee3",
    //       "is_active": true,
    //       "last_run": "2026-11-05T15:38:00Z",
    //       "name": "Abandoned signup",
    //       "reentry": "ONCE",
    //       "sequence": 1,
    //       "status": "ACTIVE",
    //       "trigger_type": "TAG_ADDED",
    //       "triggers": [
    //         {
    //           "config": {
    //             "label": "trial"
    //           },
    //           "type": "TAG_ADDED"
    //         }
    //       ],
    //       "updated_at": "2026-11-02T16:00:00Z"
    //     },
    //     {
    //       "active_enrolments": 2077,
    //       "completed_enrolments": 4120,
    //       "created_at": "2026-06-28T16:00:00Z",
    //       "description": "",
    //       "entry_step_id": "d8cdcb6a-61a1-563f-81fd-329e0ae430fd",
    //       "id": "b36c0bc7-7dae-56b4-af7f-9c1ef4673496",
    //       "is_active": true,
    //       "last_run": "2026-11-05T15:18:00Z",
    //       "name": "Re-engage · 90 days quiet",
    //       "reentry": "MULTIPLE",
    //       "sequence": 1,
    //       "status": "ACTIVE",
    //       "trigger_type": "TIME_BASED",
    //       "triggers": [
    //         {
    //           "config": {
    //             "label": "daily · segment Churn risk"
    //           },
    //           "type": "TIME_BASED"
    //         }
    //       ],
    //       "updated_at": "2026-11-02T16:00:00Z"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 6
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "active_enrolments": 1204,
      "completed_enrolments": 8311,
      "created_at": "2026-06-08T16:00:00Z",
      "description": "",
      "entry_step_id": "f1189b89-3e27-5826-bc93-e33f7450d394",
      "id": "22e49987-c27c-538a-9688-56d9ca17a7ec",
      "is_active": true,
      "last_run": "2026-11-05T15:58:00Z",
      "name": "Welcome series (7 days)",
      "reentry": "ONCE",
      "sequence": 1,
      "status": "ACTIVE",
      "trigger_type": "SUBSCRIBED_TO_LIST",
      "triggers": [
        {
          "config": {
            "label": "Newsletter"
          },
          "type": "SUBSCRIBED_TO_LIST"
        },
        {
          "config": {
            "label": "Website footer"
          },
          "type": "FORM_SUBMITTED"
        }
      ],
      "updated_at": "2026-11-02T16:00:00Z"
    },
    {
      "active_enrolments": 318,
      "completed_enrolments": 2940,
      "created_at": "2026-06-18T16:00:00Z",
      "description": "",
      "entry_step_id": "686016a9-b02e-5415-8f48-28549c0ca454",
      "id": "1114e157-f1ec-5dda-9063-79abf0255ee3",
      "is_active": true,
      "last_run": "2026-11-05T15:38:00Z",
      "name": "Abandoned signup",
      "reentry": "ONCE",
      "sequence": 1,
      "status": "ACTIVE",
      "trigger_type": "TAG_ADDED",
      "triggers": [
        {
          "config": {
            "label": "trial"
          },
          "type": "TAG_ADDED"
        }
      ],
      "updated_at": "2026-11-02T16:00:00Z"
    },
    {
      "active_enrolments": 2077,
      "completed_enrolments": 4120,
      "created_at": "2026-06-28T16:00:00Z",
      "description": "",
      "entry_step_id": "d8cdcb6a-61a1-563f-81fd-329e0ae430fd",
      "id": "b36c0bc7-7dae-56b4-af7f-9c1ef4673496",
      "is_active": true,
      "last_run": "2026-11-05T15:18:00Z",
      "name": "Re-engage · 90 days quiet",
      "reentry": "MULTIPLE",
      "sequence": 1,
      "status": "ACTIVE",
      "trigger_type": "TIME_BASED",
      "triggers": [
        {
          "config": {
            "label": "daily · segment Churn risk"
          },
          "type": "TIME_BASED"
        }
      ],
      "updated_at": "2026-11-02T16:00:00Z"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 6
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Workflow>(serde_json::json!({"active_enrolments":1204,"completed_enrolments":8311,"created_at":"2026-06-08T16:00:00Z","description":"","entry_step_id":"f1189b89-3e27-5826-bc93-e33f7450d394","id":"22e49987-c27c-538a-9688-56d9ca17a7ec","is_active":true,"last_run":"2026-11-05T15:58:00Z","name":"Welcome series (7 days)","reentry":"ONCE","sequence":1,"status":"ACTIVE","trigger_type":"SUBSCRIBED_TO_LIST","triggers":[{"config":{"label":"Newsletter"},"type":"SUBSCRIBED_TO_LIST"},{"config":{"label":"Website footer"},"type":"FORM_SUBMITTED"}],"updated_at":"2026-11-02T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Workflow>(serde_json::json!({"active_enrolments":318,"completed_enrolments":2940,"created_at":"2026-06-18T16:00:00Z","description":"","entry_step_id":"686016a9-b02e-5415-8f48-28549c0ca454","id":"1114e157-f1ec-5dda-9063-79abf0255ee3","is_active":true,"last_run":"2026-11-05T15:38:00Z","name":"Abandoned signup","reentry":"ONCE","sequence":1,"status":"ACTIVE","trigger_type":"TAG_ADDED","triggers":[{"config":{"label":"trial"},"type":"TAG_ADDED"}],"updated_at":"2026-11-02T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Workflow>(serde_json::json!({"active_enrolments":2077,"completed_enrolments":4120,"created_at":"2026-06-28T16:00:00Z","description":"","entry_step_id":"d8cdcb6a-61a1-563f-81fd-329e0ae430fd","id":"b36c0bc7-7dae-56b4-af7f-9c1ef4673496","is_active":true,"last_run":"2026-11-05T15:18:00Z","name":"Re-engage · 90 days quiet","reentry":"MULTIPLE","sequence":1,"status":"ACTIVE","trigger_type":"TIME_BASED","triggers":[{"config":{"label":"daily · segment Churn risk"},"type":"TIME_BASED"}],"updated_at":"2026-11-02T16:00:00Z"})).unwrap_or_default()],limit: 25,page: 1,total: 6,
    })
}
