// User-owned controller for handler 'list_job_errors'.

use crate::handlers::list_job_errors::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::JobError;

#[handler(ListJobErrorsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "code": "INVALID_EMAIL",
    //       "field": "email",
    //       "message": "\"jane@@example\" is not a valid address",
    //       "raw": {
    //         "Company": "Acme",
    //         "Email": "jane@@example",
    //         "First": "Jane",
    //         "Last": "Doe"
    //       },
    //       "row": 14
    //     },
    //     {
    //       "code": "DUPLICATE",
    //       "field": "email",
    //       "message": "Row 12 has the same email; dedupe on EMAIL kept the first",
    //       "raw": {
    //         "Email": "priya.natarajan@northwindtraders.example",
    //         "First": "P",
    //         "Last": "Natarajan"
    //       },
    //       "row": 88
    //     },
    //     {
    //       "code": "UNKNOWN_FIELD",
    //       "field": "plan",
    //       "message": "Column \"plan\" was not mapped to a field",
    //       "raw": {
    //         "plan": "Business"
    //       },
    //       "row": 203
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 4
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "code": "INVALID_EMAIL",
      "field": "email",
      "message": "\"jane@@example\" is not a valid address",
      "raw": {
        "Company": "Acme",
        "Email": "jane@@example",
        "First": "Jane",
        "Last": "Doe"
      },
      "row": 14
    },
    {
      "code": "DUPLICATE",
      "field": "email",
      "message": "Row 12 has the same email; dedupe on EMAIL kept the first",
      "raw": {
        "Email": "priya.natarajan@northwindtraders.example",
        "First": "P",
        "Last": "Natarajan"
      },
      "row": 88
    },
    {
      "code": "UNKNOWN_FIELD",
      "field": "plan",
      "message": "Column \"plan\" was not mapped to a field",
      "raw": {
        "plan": "Business"
      },
      "row": 203
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 4
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<JobError>(serde_json::json!({"code":"INVALID_EMAIL","field":"email","message":"\"jane@@example\" is not a valid address","raw":{"Company":"Acme","Email":"jane@@example","First":"Jane","Last":"Doe"},"row":14})).unwrap_or_default(), serde_json::from_value::<JobError>(serde_json::json!({"code":"DUPLICATE","field":"email","message":"Row 12 has the same email; dedupe on EMAIL kept the first","raw":{"Email":"priya.natarajan@northwindtraders.example","First":"P","Last":"Natarajan"},"row":88})).unwrap_or_default(), serde_json::from_value::<JobError>(serde_json::json!({"code":"UNKNOWN_FIELD","field":"plan","message":"Column \"plan\" was not mapped to a field","raw":{"plan":"Business"},"row":203})).unwrap_or_default()],limit: 20,page: 1,total: 4,
    })
}
