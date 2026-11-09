// User-owned controller for handler 'list_forms'.

use crate::handlers::list_forms::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WebForm;

#[handler(ListFormsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "actions": [
    //         {
    //           "kind": "SUBSCRIBE_LIST",
    //           "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //           "tag": "Tag",
    //           "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    //         },
    //         {
    //           "kind": "ADD_TAG",
    //           "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
    //           "tag": "newsletter",
    //           "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    //         }
    //       ],
    //       "allowed_origins": [
    //         "https://acme.example"
    //       ],
    //       "capture_fields": {},
    //       "conversion_rate": 0.013,
    //       "created_at": "2026-04-19T16:00:00Z",
    //       "description": "",
    //       "double_opt_in": true,
    //       "fields": [
    //         {
    //           "key": "email",
    //           "kind": "EMAIL",
    //           "label": "Work email",
    //           "required": true
    //         },
    //         {
    //           "key": "first_name",
    //           "kind": "TEXT",
    //           "label": "First name",
    //           "required": false
    //         },
    //         {
    //           "key": "custom:team_size",
    //           "kind": "SELECT",
    //           "label": "Team size",
    //           "options": [
    //             "1-10",
    //             "11-50",
    //             "51+"
    //           ]
    //         }
    //       ],
    //       "id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
    //       "is_active": true,
    //       "name": "Website footer",
    //       "public_url": "https://crm.acme.example/p/website-footer",
    //       "style": {
    //         "layout": "INLINE",
    //         "submit_label": "Join",
    //         "success_message": "Thanks, check your inbox to confirm."
    //       },
    //       "submission_count": 612
    //     },
    //     {
    //       "actions": [
    //         {
    //           "kind": "SUBSCRIBE_LIST",
    //           "list_id": "b3e822bf-efcc-5f58-ba14-b485ee6d6542",
    //           "tag": "Tag",
    //           "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    //         },
    //         {
    //           "kind": "ADD_TAG",
    //           "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
    //           "tag": "waitlist",
    //           "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    //         },
    //         {
    //           "kind": "START_WORKFLOW",
    //           "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
    //           "tag": "Tag",
    //           "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    //         }
    //       ],
    //       "allowed_origins": [
    //         "https://acme.example"
    //       ],
    //       "capture_fields": {},
    //       "conversion_rate": 0.117,
    //       "created_at": "2026-05-09T16:00:00Z",
    //       "description": "",
    //       "double_opt_in": true,
    //       "fields": [
    //         {
    //           "key": "email",
    //           "kind": "EMAIL",
    //           "label": "Work email",
    //           "required": true
    //         },
    //         {
    //           "key": "first_name",
    //           "kind": "TEXT",
    //           "label": "First name",
    //           "required": false
    //         },
    //         {
    //           "key": "custom:team_size",
    //           "kind": "SELECT",
    //           "label": "Team size",
    //           "options": [
    //             "1-10",
    //             "11-50",
    //             "51+"
    //           ]
    //         }
    //       ],
    //       "id": "d614ffd8-570e-5b8f-b84c-75324e12322e",
    //       "is_active": true,
    //       "name": "Launch waitlist",
    //       "public_url": "https://crm.acme.example/p/launch-waitlist",
    //       "style": {
    //         "layout": "INLINE",
    //         "submit_label": "Join",
    //         "success_message": "Thanks, check your inbox to confirm."
    //       },
    //       "submission_count": 1104
    //     },
    //     {
    //       "actions": [
    //         {
    //           "kind": "ADD_TAG",
    //           "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
    //           "tag": "trial",
    //           "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    //         },
    //         {
    //           "kind": "CREATE_LEAD",
    //           "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
    //           "tag": "Tag",
    //           "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
    //         },
    //         {
    //           "kind": "START_WORKFLOW",
    //           "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
    //           "tag": "Tag",
    //           "workflow_id": "1114e157-f1ec-5dda-9063-79abf0255ee3"
    //         }
    //       ],
    //       "allowed_origins": [
    //         "https://acme.example"
    //       ],
    //       "capture_fields": {},
    //       "conversion_rate": 0.311,
    //       "created_at": "2026-05-29T16:00:00Z",
    //       "description": "",
    //       "double_opt_in": false,
    //       "fields": [
    //         {
    //           "key": "email",
    //           "kind": "EMAIL",
    //           "label": "Work email",
    //           "required": true
    //         },
    //         {
    //           "key": "first_name",
    //           "kind": "TEXT",
    //           "label": "First name",
    //           "required": false
    //         },
    //         {
    //           "key": "custom:team_size",
    //           "kind": "SELECT",
    //           "label": "Team size",
    //           "options": [
    //             "1-10",
    //             "11-50",
    //             "51+"
    //           ]
    //         }
    //       ],
    //       "id": "997e0128-1f73-5fb4-b47e-b72197474a68",
    //       "is_active": true,
    //       "name": "Free trial",
    //       "public_url": "https://crm.acme.example/p/free-trial",
    //       "style": {
    //         "layout": "HOSTED_PAGE",
    //         "submit_label": "Join",
    //         "success_message": "Thanks, check your inbox to confirm."
    //       },
    //       "submission_count": 2140
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
      "actions": [
        {
          "kind": "SUBSCRIBE_LIST",
          "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
          "tag": "Tag",
          "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
        },
        {
          "kind": "ADD_TAG",
          "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
          "tag": "newsletter",
          "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
        }
      ],
      "allowed_origins": [
        "https://acme.example"
      ],
      "capture_fields": {},
      "conversion_rate": 0.013,
      "created_at": "2026-04-19T16:00:00Z",
      "description": "",
      "double_opt_in": true,
      "fields": [
        {
          "key": "email",
          "kind": "EMAIL",
          "label": "Work email",
          "required": true
        },
        {
          "key": "first_name",
          "kind": "TEXT",
          "label": "First name",
          "required": false
        },
        {
          "key": "custom:team_size",
          "kind": "SELECT",
          "label": "Team size",
          "options": [
            "1-10",
            "11-50",
            "51+"
          ]
        }
      ],
      "id": "454cdf11-2c7b-505e-8864-df16d18ca8cc",
      "is_active": true,
      "name": "Website footer",
      "public_url": "https://crm.acme.example/p/website-footer",
      "style": {
        "layout": "INLINE",
        "submit_label": "Join",
        "success_message": "Thanks, check your inbox to confirm."
      },
      "submission_count": 612
    },
    {
      "actions": [
        {
          "kind": "SUBSCRIBE_LIST",
          "list_id": "b3e822bf-efcc-5f58-ba14-b485ee6d6542",
          "tag": "Tag",
          "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
        },
        {
          "kind": "ADD_TAG",
          "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
          "tag": "waitlist",
          "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
        },
        {
          "kind": "START_WORKFLOW",
          "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
          "tag": "Tag",
          "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
        }
      ],
      "allowed_origins": [
        "https://acme.example"
      ],
      "capture_fields": {},
      "conversion_rate": 0.117,
      "created_at": "2026-05-09T16:00:00Z",
      "description": "",
      "double_opt_in": true,
      "fields": [
        {
          "key": "email",
          "kind": "EMAIL",
          "label": "Work email",
          "required": true
        },
        {
          "key": "first_name",
          "kind": "TEXT",
          "label": "First name",
          "required": false
        },
        {
          "key": "custom:team_size",
          "kind": "SELECT",
          "label": "Team size",
          "options": [
            "1-10",
            "11-50",
            "51+"
          ]
        }
      ],
      "id": "d614ffd8-570e-5b8f-b84c-75324e12322e",
      "is_active": true,
      "name": "Launch waitlist",
      "public_url": "https://crm.acme.example/p/launch-waitlist",
      "style": {
        "layout": "INLINE",
        "submit_label": "Join",
        "success_message": "Thanks, check your inbox to confirm."
      },
      "submission_count": 1104
    },
    {
      "actions": [
        {
          "kind": "ADD_TAG",
          "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
          "tag": "trial",
          "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
        },
        {
          "kind": "CREATE_LEAD",
          "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
          "tag": "Tag",
          "workflow_id": "4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"
        },
        {
          "kind": "START_WORKFLOW",
          "list_id": "c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b",
          "tag": "Tag",
          "workflow_id": "1114e157-f1ec-5dda-9063-79abf0255ee3"
        }
      ],
      "allowed_origins": [
        "https://acme.example"
      ],
      "capture_fields": {},
      "conversion_rate": 0.311,
      "created_at": "2026-05-29T16:00:00Z",
      "description": "",
      "double_opt_in": false,
      "fields": [
        {
          "key": "email",
          "kind": "EMAIL",
          "label": "Work email",
          "required": true
        },
        {
          "key": "first_name",
          "kind": "TEXT",
          "label": "First name",
          "required": false
        },
        {
          "key": "custom:team_size",
          "kind": "SELECT",
          "label": "Team size",
          "options": [
            "1-10",
            "11-50",
            "51+"
          ]
        }
      ],
      "id": "997e0128-1f73-5fb4-b47e-b72197474a68",
      "is_active": true,
      "name": "Free trial",
      "public_url": "https://crm.acme.example/p/free-trial",
      "style": {
        "layout": "HOSTED_PAGE",
        "submit_label": "Join",
        "success_message": "Thanks, check your inbox to confirm."
      },
      "submission_count": 2140
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
        items: vec![serde_json::from_value::<WebForm>(serde_json::json!({"actions":[{"kind":"SUBSCRIBE_LIST","list_id":"09f5b2f9-2adc-5edd-a6fd-c20675680671","tag":"Tag","workflow_id":"4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"},{"kind":"ADD_TAG","list_id":"c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b","tag":"newsletter","workflow_id":"4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"}],"allowed_origins":["https://acme.example"],"capture_fields":{},"conversion_rate":0.013,"created_at":"2026-04-19T16:00:00Z","description":"","double_opt_in":true,"fields":[{"key":"email","kind":"EMAIL","label":"Work email","required":true},{"key":"first_name","kind":"TEXT","label":"First name","required":false},{"key":"custom:team_size","kind":"SELECT","label":"Team size","options":["1-10","11-50","51+"]}],"id":"454cdf11-2c7b-505e-8864-df16d18ca8cc","is_active":true,"name":"Website footer","public_url":"https://crm.acme.example/p/website-footer","style":{"layout":"INLINE","submit_label":"Join","success_message":"Thanks, check your inbox to confirm."},"submission_count":612})).unwrap_or_default(), serde_json::from_value::<WebForm>(serde_json::json!({"actions":[{"kind":"SUBSCRIBE_LIST","list_id":"b3e822bf-efcc-5f58-ba14-b485ee6d6542","tag":"Tag","workflow_id":"4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"},{"kind":"ADD_TAG","list_id":"c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b","tag":"waitlist","workflow_id":"4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"},{"kind":"START_WORKFLOW","list_id":"c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b","tag":"Tag","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec"}],"allowed_origins":["https://acme.example"],"capture_fields":{},"conversion_rate":0.117,"created_at":"2026-05-09T16:00:00Z","description":"","double_opt_in":true,"fields":[{"key":"email","kind":"EMAIL","label":"Work email","required":true},{"key":"first_name","kind":"TEXT","label":"First name","required":false},{"key":"custom:team_size","kind":"SELECT","label":"Team size","options":["1-10","11-50","51+"]}],"id":"d614ffd8-570e-5b8f-b84c-75324e12322e","is_active":true,"name":"Launch waitlist","public_url":"https://crm.acme.example/p/launch-waitlist","style":{"layout":"INLINE","submit_label":"Join","success_message":"Thanks, check your inbox to confirm."},"submission_count":1104})).unwrap_or_default(), serde_json::from_value::<WebForm>(serde_json::json!({"actions":[{"kind":"ADD_TAG","list_id":"c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b","tag":"trial","workflow_id":"4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"},{"kind":"CREATE_LEAD","list_id":"c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b","tag":"Tag","workflow_id":"4fcbd1ca-b33b-5575-ab91-494f0d8bd9f4"},{"kind":"START_WORKFLOW","list_id":"c8ef0d2c-4e5a-58ea-bd07-c5b19097d04b","tag":"Tag","workflow_id":"1114e157-f1ec-5dda-9063-79abf0255ee3"}],"allowed_origins":["https://acme.example"],"capture_fields":{},"conversion_rate":0.311,"created_at":"2026-05-29T16:00:00Z","description":"","double_opt_in":false,"fields":[{"key":"email","kind":"EMAIL","label":"Work email","required":true},{"key":"first_name","kind":"TEXT","label":"First name","required":false},{"key":"custom:team_size","kind":"SELECT","label":"Team size","options":["1-10","11-50","51+"]}],"id":"997e0128-1f73-5fb4-b47e-b72197474a68","is_active":true,"name":"Free trial","public_url":"https://crm.acme.example/p/free-trial","style":{"layout":"HOSTED_PAGE","submit_label":"Join","success_message":"Thanks, check your inbox to confirm."},"submission_count":2140})).unwrap_or_default()],limit: 25,page: 1,total: 6,
    })
}
