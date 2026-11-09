// User-owned controller for handler 'preview_campaign_audience'.

use crate::handlers::preview_campaign_audience::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PreviewCampaignAudienceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "count": 11312,
    //   "excluded_by_segment": 0,
    //   "excluded_suppressed": 464,
    //   "excluded_unsubscribed": 1104,
    //   "sample": [
    //     {
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "email": "priya.natarajan@northwindtraders.example",
    //       "name": "Priya Natarajan"
    //     },
    //     {
    //       "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "email": "tom.okafor@fabrikam.example",
    //       "name": "Tom Okafor"
    //     },
    //     {
    //       "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "email": "lena.fischer@contoso.example",
    //       "name": "Lena Fischer"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "count": 11312,
  "excluded_by_segment": 0,
  "excluded_suppressed": 464,
  "excluded_unsubscribed": 1104,
  "sample": [
    {
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "email": "priya.natarajan@northwindtraders.example",
      "name": "Priya Natarajan"
    },
    {
      "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "email": "tom.okafor@fabrikam.example",
      "name": "Tom Okafor"
    },
    {
      "contact_id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "email": "lena.fischer@contoso.example",
      "name": "Lena Fischer"
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
        count: 11312,excluded_by_segment: Some(0),excluded_suppressed: Some(464),excluded_unsubscribed: Some(1104),sample: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","email":"priya.natarajan@northwindtraders.example","name":"Priya Natarajan"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","email":"tom.okafor@fabrikam.example","name":"Tom Okafor"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","email":"lena.fischer@contoso.example","name":"Lena Fischer"}"#).unwrap_or(serde_json::Value::Null)]),
    })
}
