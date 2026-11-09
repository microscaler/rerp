// User-owned controller for handler 'list_ad_conversions'.

use crate::handlers::list_ad_conversions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdConversion;

#[handler(ListAdConversionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "click_id_param": "Click id param",
    //       "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "contact_name": "Priya Natarajan",
    //       "currency": "EUR",
    //       "error": "Error",
    //       "event": "FORM_SUBMITTED",
    //       "external_event_name": "Lead",
    //       "id": "80c7cfa1-89bb-5075-98d3-ae39551fedde",
    //       "identifiers_sent": [
    //         "EMAIL",
    //         "PHONE"
    //       ],
    //       "occurred_at": "2026-11-05T14:00:00Z",
    //       "platform": "META",
    //       "rule_id": "6d83f7aa-792a-54cd-a144-a8810febc825",
    //       "sent_at": "2026-11-05T15:00:00Z",
    //       "status": "SENT",
    //       "value": "40.00"
    //     },
    //     {
    //       "account_id": "1dcd203d-8734-5d40-b43b-9bbcdd9758b5",
    //       "click_id_param": "gclid",
    //       "contact_id": "93f1db66-d1af-580b-b546-6a13f1774d12",
    //       "contact_name": "Marcus Reid",
    //       "currency": "EUR",
    //       "error": "Error",
    //       "event": "DEAL_WON",
    //       "external_event_name": "Purchase",
    //       "id": "991cc3a1-fb48-51b8-a9ce-8c5bc0a7f864",
    //       "identifiers_sent": [
    //         "EMAIL",
    //         "PHONE"
    //       ],
    //       "occurred_at": "2026-11-05T13:00:00Z",
    //       "platform": "GOOGLE_ADS",
    //       "rule_id": "396b3d50-b9b4-500f-a867-7df574e7825f",
    //       "sent_at": "2026-11-05T14:00:00Z",
    //       "status": "SENT",
    //       "value": "38400.00"
    //     },
    //     {
    //       "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
    //       "click_id_param": "Click id param",
    //       "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "contact_name": "Tom Okafor",
    //       "currency": "EUR",
    //       "error": "Platform rejected: event time older than 7 days",
    //       "event": "FORM_SUBMITTED",
    //       "external_event_name": "Lead",
    //       "id": "8080102f-a8ee-5b1b-9ca1-cfa9668d411b",
    //       "identifiers_sent": [
    //         "EMAIL",
    //         "PHONE"
    //       ],
    //       "occurred_at": "2026-11-05T10:00:00Z",
    //       "platform": "LINKEDIN",
    //       "rule_id": "6d83f7aa-792a-54cd-a144-a8810febc825",
    //       "sent_at": "2026-11-05T16:00:00Z",
    //       "status": "FAILED",
    //       "value": "40.00"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 1318
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
      "click_id_param": "Click id param",
      "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "contact_name": "Priya Natarajan",
      "currency": "EUR",
      "error": "Error",
      "event": "FORM_SUBMITTED",
      "external_event_name": "Lead",
      "id": "80c7cfa1-89bb-5075-98d3-ae39551fedde",
      "identifiers_sent": [
        "EMAIL",
        "PHONE"
      ],
      "occurred_at": "2026-11-05T14:00:00Z",
      "platform": "META",
      "rule_id": "6d83f7aa-792a-54cd-a144-a8810febc825",
      "sent_at": "2026-11-05T15:00:00Z",
      "status": "SENT",
      "value": "40.00"
    },
    {
      "account_id": "1dcd203d-8734-5d40-b43b-9bbcdd9758b5",
      "click_id_param": "gclid",
      "contact_id": "93f1db66-d1af-580b-b546-6a13f1774d12",
      "contact_name": "Marcus Reid",
      "currency": "EUR",
      "error": "Error",
      "event": "DEAL_WON",
      "external_event_name": "Purchase",
      "id": "991cc3a1-fb48-51b8-a9ce-8c5bc0a7f864",
      "identifiers_sent": [
        "EMAIL",
        "PHONE"
      ],
      "occurred_at": "2026-11-05T13:00:00Z",
      "platform": "GOOGLE_ADS",
      "rule_id": "396b3d50-b9b4-500f-a867-7df574e7825f",
      "sent_at": "2026-11-05T14:00:00Z",
      "status": "SENT",
      "value": "38400.00"
    },
    {
      "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
      "click_id_param": "Click id param",
      "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "contact_name": "Tom Okafor",
      "currency": "EUR",
      "error": "Platform rejected: event time older than 7 days",
      "event": "FORM_SUBMITTED",
      "external_event_name": "Lead",
      "id": "8080102f-a8ee-5b1b-9ca1-cfa9668d411b",
      "identifiers_sent": [
        "EMAIL",
        "PHONE"
      ],
      "occurred_at": "2026-11-05T10:00:00Z",
      "platform": "LINKEDIN",
      "rule_id": "6d83f7aa-792a-54cd-a144-a8810febc825",
      "sent_at": "2026-11-05T16:00:00Z",
      "status": "FAILED",
      "value": "40.00"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 1318
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdConversion>(serde_json::json!({"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","click_id_param":"Click id param","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","contact_name":"Priya Natarajan","currency":"EUR","error":"Error","event":"FORM_SUBMITTED","external_event_name":"Lead","id":"80c7cfa1-89bb-5075-98d3-ae39551fedde","identifiers_sent":["EMAIL","PHONE"],"occurred_at":"2026-11-05T14:00:00Z","platform":"META","rule_id":"6d83f7aa-792a-54cd-a144-a8810febc825","sent_at":"2026-11-05T15:00:00Z","status":"SENT","value":"40.00"})).unwrap_or_default(), serde_json::from_value::<AdConversion>(serde_json::json!({"account_id":"1dcd203d-8734-5d40-b43b-9bbcdd9758b5","click_id_param":"gclid","contact_id":"93f1db66-d1af-580b-b546-6a13f1774d12","contact_name":"Marcus Reid","currency":"EUR","error":"Error","event":"DEAL_WON","external_event_name":"Purchase","id":"991cc3a1-fb48-51b8-a9ce-8c5bc0a7f864","identifiers_sent":["EMAIL","PHONE"],"occurred_at":"2026-11-05T13:00:00Z","platform":"GOOGLE_ADS","rule_id":"396b3d50-b9b4-500f-a867-7df574e7825f","sent_at":"2026-11-05T14:00:00Z","status":"SENT","value":"38400.00"})).unwrap_or_default(), serde_json::from_value::<AdConversion>(serde_json::json!({"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","click_id_param":"Click id param","contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","contact_name":"Tom Okafor","currency":"EUR","error":"Platform rejected: event time older than 7 days","event":"FORM_SUBMITTED","external_event_name":"Lead","id":"8080102f-a8ee-5b1b-9ca1-cfa9668d411b","identifiers_sent":["EMAIL","PHONE"],"occurred_at":"2026-11-05T10:00:00Z","platform":"LINKEDIN","rule_id":"6d83f7aa-792a-54cd-a144-a8810febc825","sent_at":"2026-11-05T16:00:00Z","status":"FAILED","value":"40.00"})).unwrap_or_default()],limit: 25,page: 1,total: 1318,
    })
}
