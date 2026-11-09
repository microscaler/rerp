// User-owned controller for handler 'list_deal_contacts'.

use crate::handlers::list_deal_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealContact;

#[handler(ListDealContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "email": "priya.natarajan@northwindtraders.example",
    //       "name": "Priya Natarajan",
    //       "role": "Decision maker"
    //     },
    //     {
    //       "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "email": "tom.okafor@fabrikam.example",
    //       "name": "Tom Okafor",
    //       "role": "Champion"
    //     },
    //     {
    //       "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "email": "lena.fischer@contoso.example",
    //       "name": "Lena Fischer",
    //       "role": "Procurement"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "email": "priya.natarajan@northwindtraders.example",
      "name": "Priya Natarajan",
      "role": "Decision maker"
    },
    {
      "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "email": "tom.okafor@fabrikam.example",
      "name": "Tom Okafor",
      "role": "Champion"
    },
    {
      "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "email": "lena.fischer@contoso.example",
      "name": "Lena Fischer",
      "role": "Procurement"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<DealContact>(serde_json::json!({"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","email":"priya.natarajan@northwindtraders.example","name":"Priya Natarajan","role":"Decision maker"})).unwrap_or_default(), serde_json::from_value::<DealContact>(serde_json::json!({"contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","email":"tom.okafor@fabrikam.example","name":"Tom Okafor","role":"Champion"})).unwrap_or_default(), serde_json::from_value::<DealContact>(serde_json::json!({"contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","email":"lena.fischer@contoso.example","name":"Lena Fischer","role":"Procurement"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
