// User-owned controller for handler 'get_automation_contacts'.

use crate::handlers::get_automation_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AutomationContactRowPage;
#[allow(unused_imports)]
use crate::handlers::types::CompletionBucket;
#[allow(unused_imports)]
use crate::handlers::types::NameCount;

#[handler(GetAutomationContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "avg_seconds_to_complete": 622080,
    //   "completion_rate": 0.855,
    //   "completion_rate_change_30d": 0.031,
    //   "completion_times": [
    //     {
    //       "count": 112,
    //       "label": "< 1 day",
    //       "max_seconds": 86400,
    //       "min_seconds": 0
    //     },
    //     {
    //       "count": 240,
    //       "label": "1-3 days",
    //       "max_seconds": 259200,
    //       "min_seconds": 86400
    //     },
    //     {
    //       "count": 806,
    //       "label": "3-7 days",
    //       "max_seconds": 604800,
    //       "min_seconds": 259200
    //     }
    //   ],
    //   "contacts": {
    //     "items": [
    //       {
    //         "completed_at": "2026-11-04T16:00:00Z",
    //         "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //         "current_step": null,
    //         "email": "priya.natarajan@northwindtraders.example",
    //         "entered_at": "2026-10-28T16:00:00Z",
    //         "name": "Priya Natarajan",
    //         "status": "COMPLETED"
    //       },
    //       {
    //         "completed_at": "2026-11-05T16:00:00Z",
    //         "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //         "current_step": "Wait 2 days → Day 3 tips",
    //         "email": "tom.okafor@fabrikam.example",
    //         "entered_at": "2026-11-02T16:00:00Z",
    //         "name": "Tom Okafor",
    //         "status": "ACTIVE"
    //       },
    //       {
    //         "completed_at": "2026-11-05T16:00:00Z",
    //         "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //         "current_step": null,
    //         "email": "lena.fischer@contoso.example",
    //         "entered_at": "2026-10-29T16:00:00Z",
    //         "name": "Lena Fischer",
    //         "status": "EXITED"
    //       }
    //     ],
    //     "limit": 20,
    //     "page": 1,
    //     "total": 1640
    //   },
    //   "contacts_entered": 1640,
    //   "top_lists": [
    //     {
    //       "count": 92,
    //       "name": "Webinars"
    //     },
    //     {
    //       "count": 86,
    //       "name": "Newsletter"
    //     },
    //     {
    //       "count": 79,
    //       "name": "Product launch waitlist"
    //     }
    //   ],
    //   "top_tags": [
    //     {
    //       "count": 94,
    //       "name": "webinar"
    //     },
    //     {
    //       "count": 91,
    //       "name": "pricing-page"
    //     },
    //     {
    //       "count": 88,
    //       "name": "trial"
    //     }
    //   ],
    //   "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "avg_seconds_to_complete": 622080,
  "completion_rate": 0.855,
  "completion_rate_change_30d": 0.031,
  "completion_times": [
    {
      "count": 112,
      "label": "< 1 day",
      "max_seconds": 86400,
      "min_seconds": 0
    },
    {
      "count": 240,
      "label": "1-3 days",
      "max_seconds": 259200,
      "min_seconds": 86400
    },
    {
      "count": 806,
      "label": "3-7 days",
      "max_seconds": 604800,
      "min_seconds": 259200
    }
  ],
  "contacts": {
    "items": [
      {
        "completed_at": "2026-11-04T16:00:00Z",
        "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
        "current_step": null,
        "email": "priya.natarajan@northwindtraders.example",
        "entered_at": "2026-10-28T16:00:00Z",
        "name": "Priya Natarajan",
        "status": "COMPLETED"
      },
      {
        "completed_at": "2026-11-05T16:00:00Z",
        "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
        "current_step": "Wait 2 days → Day 3 tips",
        "email": "tom.okafor@fabrikam.example",
        "entered_at": "2026-11-02T16:00:00Z",
        "name": "Tom Okafor",
        "status": "ACTIVE"
      },
      {
        "completed_at": "2026-11-05T16:00:00Z",
        "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
        "current_step": null,
        "email": "lena.fischer@contoso.example",
        "entered_at": "2026-10-29T16:00:00Z",
        "name": "Lena Fischer",
        "status": "EXITED"
      }
    ],
    "limit": 20,
    "page": 1,
    "total": 1640
  },
  "contacts_entered": 1640,
  "top_lists": [
    {
      "count": 92,
      "name": "Webinars"
    },
    {
      "count": 86,
      "name": "Newsletter"
    },
    {
      "count": 79,
      "name": "Product launch waitlist"
    }
  ],
  "top_tags": [
    {
      "count": 94,
      "name": "webinar"
    },
    {
      "count": 91,
      "name": "pricing-page"
    },
    {
      "count": 88,
      "name": "trial"
    }
  ],
  "workflow_id": "22e49987-c27c-538a-9688-56d9ca17a7ec"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        avg_seconds_to_complete: Some(622080),completion_rate: 0.855,completion_rate_change_30d: Some(0.031),completion_times: Some(vec![serde_json::from_value::<CompletionBucket>(serde_json::json!({"count":112,"label":"< 1 day","max_seconds":86400,"min_seconds":0})).unwrap_or_default(), serde_json::from_value::<CompletionBucket>(serde_json::json!({"count":240,"label":"1-3 days","max_seconds":259200,"min_seconds":86400})).unwrap_or_default(), serde_json::from_value::<CompletionBucket>(serde_json::json!({"count":806,"label":"3-7 days","max_seconds":604800,"min_seconds":259200})).unwrap_or_default()]),contacts: Some(serde_json::from_value::<AutomationContactRowPage>(serde_json::json!({"items":[{"completed_at":"2026-11-04T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","current_step":null,"email":"priya.natarajan@northwindtraders.example","entered_at":"2026-10-28T16:00:00Z","name":"Priya Natarajan","status":"COMPLETED"},{"completed_at":"2026-11-05T16:00:00Z","contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","current_step":"Wait 2 days → Day 3 tips","email":"tom.okafor@fabrikam.example","entered_at":"2026-11-02T16:00:00Z","name":"Tom Okafor","status":"ACTIVE"},{"completed_at":"2026-11-05T16:00:00Z","contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","current_step":null,"email":"lena.fischer@contoso.example","entered_at":"2026-10-29T16:00:00Z","name":"Lena Fischer","status":"EXITED"}],"limit":20,"page":1,"total":1640})).unwrap_or_default()),contacts_entered: 1640,top_lists: Some(vec![serde_json::from_value::<NameCount>(serde_json::json!({"count":92,"name":"Webinars"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":86,"name":"Newsletter"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":79,"name":"Product launch waitlist"})).unwrap_or_default()]),top_tags: Some(vec![serde_json::from_value::<NameCount>(serde_json::json!({"count":94,"name":"webinar"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":91,"name":"pricing-page"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":88,"name":"trial"})).unwrap_or_default()]),workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(),
    })
}
