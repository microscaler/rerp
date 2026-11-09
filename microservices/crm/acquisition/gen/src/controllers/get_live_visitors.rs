// User-owned controller for handler 'get_live_visitors'.

use crate::handlers::get_live_visitors::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetLiveVisitorsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "at": "2026-11-05T15:59:48Z",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "contact_name": "Priya Natarajan",
    //       "page": "/pricing",
    //       "source": "email · oct-product-update"
    //     },
    //     {
    //       "at": "2026-11-05T15:58:00Z",
    //       "contact_id": "5de76105-f8c3-5357-af13-c5ce28753ed0",
    //       "contact_name": "Jonas Weber",
    //       "source": "app"
    //     },
    //     {
    //       "at": "2026-11-05T15:54:00Z",
    //       "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "contact_name": "Lena Fischer",
    //       "page": "/launch",
    //       "source": "google · cpc"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "at": "2026-11-05T15:59:48Z",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "contact_name": "Priya Natarajan",
      "page": "/pricing",
      "source": "email · oct-product-update"
    },
    {
      "at": "2026-11-05T15:58:00Z",
      "contact_id": "5de76105-f8c3-5357-af13-c5ce28753ed0",
      "contact_name": "Jonas Weber",
      "source": "app"
    },
    {
      "at": "2026-11-05T15:54:00Z",
      "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "contact_name": "Lena Fischer",
      "page": "/launch",
      "source": "google · cpc"
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

    HttpJson::ok(Response {
        anonymous_count: Some(42),items: vec![serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-11-05T15:59:48Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","contact_name":"Priya Natarajan","page":"/pricing","source":"email · oct-product-update"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-11-05T15:58:00Z","contact_id":"5de76105-f8c3-5357-af13-c5ce28753ed0","contact_name":"Jonas Weber","source":"app"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"at":"2026-11-05T15:54:00Z","contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","contact_name":"Lena Fischer","page":"/launch","source":"google · cpc"}"#).unwrap_or(serde_json::Value::Null)],
    })
}
