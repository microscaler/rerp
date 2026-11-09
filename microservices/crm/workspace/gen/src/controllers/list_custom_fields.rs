// User-owned controller for handler 'list_custom_fields'.

use crate::handlers::list_custom_fields::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CustomField;

#[handler(ListCustomFieldsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "entity": "contact",
    //       "group": "Profile",
    //       "hint": "",
    //       "id": "9851eff3-bcf4-59e1-9c43-82ba49bdc08c",
    //       "key": "plan",
    //       "label": "Plan",
    //       "merge_field": "custom.plan",
    //       "options": {},
    //       "required": false,
    //       "searchable": false,
    //       "sequence": 1,
    //       "show_on_card": false,
    //       "type": "SELECT",
    //       "visible": false
    //     },
    //     {
    //       "entity": "contact",
    //       "group": "Profile",
    //       "hint": "",
    //       "id": "6cb8ed49-8877-5cc2-bc25-eafe10a4a357",
    //       "key": "renewal_date",
    //       "label": "Renewal date",
    //       "merge_field": "custom.renewal_date",
    //       "required": false,
    //       "searchable": false,
    //       "sequence": 2,
    //       "show_on_card": false,
    //       "type": "DATE",
    //       "visible": false
    //     },
    //     {
    //       "entity": "contact",
    //       "group": "Profile",
    //       "hint": "",
    //       "id": "8b2236e2-bcc8-589a-baaf-fa871172e5ee",
    //       "key": "seats",
    //       "label": "Seats",
    //       "merge_field": "custom.seats",
    //       "required": false,
    //       "searchable": false,
    //       "sequence": 3,
    //       "show_on_card": false,
    //       "type": "STRING",
    //       "visible": false
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 20
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "entity": "contact",
      "group": "Profile",
      "hint": "",
      "id": "9851eff3-bcf4-59e1-9c43-82ba49bdc08c",
      "key": "plan",
      "label": "Plan",
      "merge_field": "custom.plan",
      "options": {},
      "required": false,
      "searchable": false,
      "sequence": 1,
      "show_on_card": false,
      "type": "SELECT",
      "visible": false
    },
    {
      "entity": "contact",
      "group": "Profile",
      "hint": "",
      "id": "6cb8ed49-8877-5cc2-bc25-eafe10a4a357",
      "key": "renewal_date",
      "label": "Renewal date",
      "merge_field": "custom.renewal_date",
      "required": false,
      "searchable": false,
      "sequence": 2,
      "show_on_card": false,
      "type": "DATE",
      "visible": false
    },
    {
      "entity": "contact",
      "group": "Profile",
      "hint": "",
      "id": "8b2236e2-bcc8-589a-baaf-fa871172e5ee",
      "key": "seats",
      "label": "Seats",
      "merge_field": "custom.seats",
      "required": false,
      "searchable": false,
      "sequence": 3,
      "show_on_card": false,
      "type": "STRING",
      "visible": false
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 20
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<CustomField>(serde_json::json!({"entity":"contact","group":"Profile","hint":"","id":"9851eff3-bcf4-59e1-9c43-82ba49bdc08c","key":"plan","label":"Plan","merge_field":"custom.plan","options":{},"required":false,"searchable":false,"sequence":1,"show_on_card":false,"type":"SELECT","visible":false})).unwrap_or_default(), serde_json::from_value::<CustomField>(serde_json::json!({"entity":"contact","group":"Profile","hint":"","id":"6cb8ed49-8877-5cc2-bc25-eafe10a4a357","key":"renewal_date","label":"Renewal date","merge_field":"custom.renewal_date","required":false,"searchable":false,"sequence":2,"show_on_card":false,"type":"DATE","visible":false})).unwrap_or_default(), serde_json::from_value::<CustomField>(serde_json::json!({"entity":"contact","group":"Profile","hint":"","id":"8b2236e2-bcc8-589a-baaf-fa871172e5ee","key":"seats","label":"Seats","merge_field":"custom.seats","required":false,"searchable":false,"sequence":3,"show_on_card":false,"type":"STRING","visible":false})).unwrap_or_default()],limit: 25,page: 1,total: 20,
    })
}
