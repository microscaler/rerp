// User-owned controller for handler 'get_nearby_contacts'.

use crate::handlers::get_nearby_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NearbyContact;

#[handler(GetNearbyContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "city": "Sofia",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "country": "BG",
    //       "distance": 2.1,
    //       "email": "priya.natarajan@northwindtraders.example",
    //       "name": "Priya Natarajan"
    //     },
    //     {
    //       "city": "Sofia",
    //       "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "country": "BG",
    //       "distance": 6.8,
    //       "email": "tom.okafor@fabrikam.example",
    //       "name": "Tom Okafor"
    //     },
    //     {
    //       "city": "Sofia",
    //       "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "country": "BG",
    //       "distance": 14.2,
    //       "email": "lena.fischer@contoso.example",
    //       "name": "Lena Fischer"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "city": "Sofia",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "country": "BG",
      "distance": 2.1,
      "email": "priya.natarajan@northwindtraders.example",
      "name": "Priya Natarajan"
    },
    {
      "city": "Sofia",
      "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "country": "BG",
      "distance": 6.8,
      "email": "tom.okafor@fabrikam.example",
      "name": "Tom Okafor"
    },
    {
      "city": "Sofia",
      "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "country": "BG",
      "distance": 14.2,
      "email": "lena.fischer@contoso.example",
      "name": "Lena Fischer"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 3
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<NearbyContact>(serde_json::json!({"city":"Sofia","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","country":"BG","distance":2.1,"email":"priya.natarajan@northwindtraders.example","name":"Priya Natarajan"})).unwrap_or_default(), serde_json::from_value::<NearbyContact>(serde_json::json!({"city":"Sofia","contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","country":"BG","distance":6.8,"email":"tom.okafor@fabrikam.example","name":"Tom Okafor"})).unwrap_or_default(), serde_json::from_value::<NearbyContact>(serde_json::json!({"city":"Sofia","contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","country":"BG","distance":14.2,"email":"lena.fischer@contoso.example","name":"Lena Fischer"})).unwrap_or_default()],limit: 20,page: 1,total: 3,
    })
}
