// User-owned controller for handler 'list_segments'.

use crate::handlers::list_segments::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Segment;

#[handler(ListSegmentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "conditions": {
    //         "items": [
    //           {
    //             "operator": "HAS",
    //             "subject": "engagement:any:30"
    //           }
    //         ],
    //         "op": "AND"
    //       },
    //       "contact_count": 6204,
    //       "created_at": "2026-08-07T16:00:00Z",
    //       "description": "Opened or clicked in the last 30 days",
    //       "id": "20df9eb1-f769-5766-b664-2f6b6529022c",
    //       "name": "Engaged · 30 days"
    //     },
    //     {
    //       "conditions": {
    //         "items": [
    //           {
    //             "operator": "HAS",
    //             "subject": "tag",
    //             "value": "trial"
    //           },
    //           {
    //             "operator": "NOT_WITHIN_DAYS",
    //             "subject": "event:activated",
    //             "value": 14
    //           },
    //           {
    //             "operator": "WITHIN_DAYS",
    //             "subject": "created_at",
    //             "value": 30
    //           }
    //         ],
    //         "op": "AND"
    //       },
    //       "contact_count": 412,
    //       "created_at": "2026-09-06T16:00:00Z",
    //       "description": "",
    //       "id": "347521ff-872c-5cea-a916-4b0673fc7b2f",
    //       "name": "Trial · no activation"
    //     },
    //     {
    //       "conditions": {
    //         "items": [
    //           {
    //             "operator": "HAS",
    //             "subject": "tag",
    //             "value": "customer"
    //           }
    //         ],
    //         "op": "AND"
    //       },
    //       "contact_count": 1820,
    //       "created_at": "2026-04-19T16:00:00Z",
    //       "description": "",
    //       "id": "3fa577a2-af26-5107-ad12-4ae022268d0f",
    //       "name": "Customers"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 5
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "conditions": {
        "items": [
          {
            "operator": "HAS",
            "subject": "engagement:any:30"
          }
        ],
        "op": "AND"
      },
      "contact_count": 6204,
      "created_at": "2026-08-07T16:00:00Z",
      "description": "Opened or clicked in the last 30 days",
      "id": "20df9eb1-f769-5766-b664-2f6b6529022c",
      "name": "Engaged · 30 days"
    },
    {
      "conditions": {
        "items": [
          {
            "operator": "HAS",
            "subject": "tag",
            "value": "trial"
          },
          {
            "operator": "NOT_WITHIN_DAYS",
            "subject": "event:activated",
            "value": 14
          },
          {
            "operator": "WITHIN_DAYS",
            "subject": "created_at",
            "value": 30
          }
        ],
        "op": "AND"
      },
      "contact_count": 412,
      "created_at": "2026-09-06T16:00:00Z",
      "description": "",
      "id": "347521ff-872c-5cea-a916-4b0673fc7b2f",
      "name": "Trial · no activation"
    },
    {
      "conditions": {
        "items": [
          {
            "operator": "HAS",
            "subject": "tag",
            "value": "customer"
          }
        ],
        "op": "AND"
      },
      "contact_count": 1820,
      "created_at": "2026-04-19T16:00:00Z",
      "description": "",
      "id": "3fa577a2-af26-5107-ad12-4ae022268d0f",
      "name": "Customers"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 5
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Segment>(serde_json::json!({"conditions":{"items":[{"operator":"HAS","subject":"engagement:any:30"}],"op":"AND"},"contact_count":6204,"created_at":"2026-08-07T16:00:00Z","description":"Opened or clicked in the last 30 days","id":"20df9eb1-f769-5766-b664-2f6b6529022c","name":"Engaged · 30 days"})).unwrap_or_default(), serde_json::from_value::<Segment>(serde_json::json!({"conditions":{"items":[{"operator":"HAS","subject":"tag","value":"trial"},{"operator":"NOT_WITHIN_DAYS","subject":"event:activated","value":14},{"operator":"WITHIN_DAYS","subject":"created_at","value":30}],"op":"AND"},"contact_count":412,"created_at":"2026-09-06T16:00:00Z","description":"","id":"347521ff-872c-5cea-a916-4b0673fc7b2f","name":"Trial · no activation"})).unwrap_or_default(), serde_json::from_value::<Segment>(serde_json::json!({"conditions":{"items":[{"operator":"HAS","subject":"tag","value":"customer"}],"op":"AND"},"contact_count":1820,"created_at":"2026-04-19T16:00:00Z","description":"","id":"3fa577a2-af26-5107-ad12-4ae022268d0f","name":"Customers"})).unwrap_or_default()],limit: 25,page: 1,total: 5,
    })
}
