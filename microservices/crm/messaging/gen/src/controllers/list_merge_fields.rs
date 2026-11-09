// User-owned controller for handler 'list_merge_fields'.

use crate::handlers::list_merge_fields::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::MergeField;

#[handler(ListMergeFieldsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "group": "contact",
    //       "key": "contact.first_name",
    //       "label": "First name",
    //       "sample": "Priya"
    //     },
    //     {
    //       "group": "contact",
    //       "key": "contact.last_name",
    //       "label": "Last name",
    //       "sample": "Natarajan"
    //     },
    //     {
    //       "group": "contact",
    //       "key": "contact.email",
    //       "label": "Email",
    //       "sample": "priya@example.com"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "group": "contact",
      "key": "contact.first_name",
      "label": "First name",
      "sample": "Priya"
    },
    {
      "group": "contact",
      "key": "contact.last_name",
      "label": "Last name",
      "sample": "Natarajan"
    },
    {
      "group": "contact",
      "key": "contact.email",
      "label": "Email",
      "sample": "priya@example.com"
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

    HttpJson::ok(Response(vec![serde_json::from_value::<MergeField>(serde_json::json!({"group":"contact","key":"contact.first_name","label":"First name","sample":"Priya"})).unwrap_or_default(), serde_json::from_value::<MergeField>(serde_json::json!({"group":"contact","key":"contact.last_name","label":"Last name","sample":"Natarajan"})).unwrap_or_default(), serde_json::from_value::<MergeField>(serde_json::json!({"group":"contact","key":"contact.email","label":"Email","sample":"priya@example.com"})).unwrap_or_default()]))
}
