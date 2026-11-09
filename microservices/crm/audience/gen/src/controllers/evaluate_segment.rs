// User-owned controller for handler 'evaluate_segment'.

use crate::handlers::evaluate_segment::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Contact;

#[handler(EvaluateSegmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "count": 412,
    //   "sample": [
    //     {
    //       "attribution": {
    //         "utm_campaign": "bf-2026",
    //         "utm_medium": "social",
    //         "utm_source": "linkedin"
    //       },
    //       "country_code": "GB",
    //       "created_at": "2026-10-20T16:00:00Z",
    //       "custom_fields": {
    //         "industry": "Retail",
    //         "plan": "Team",
    //         "renewal_date": "2026-12-14",
    //         "seats": 8,
    //         "team_size": "11-50"
    //       },
    //       "email": "tom.okafor@fabrikam.example",
    //       "first_name": "Tom",
    //       "id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "job_title": "Founder",
    //       "last_activity_at": "2026-11-05T08:00:00Z",
    //       "last_name": "Okafor",
    //       "list_ids": [
    //         "09f5b2f9-2adc-5edd-a6fd-c20675680671"
    //       ],
    //       "list_names": [
    //         "Newsletter"
    //       ],
    //       "message_stats": {
    //         "hard_bounces": 0,
    //         "last_click_at": "2026-11-03T16:00:00Z",
    //         "last_open_at": "2026-11-05T09:00:00Z",
    //         "sent": 13,
    //         "soft_bounces": 0
    //       },
    //       "organisation_id": "576a7630-a5b2-566c-ac00-caca59662cb0",
    //       "organisation_name": "Fabrikam Inc",
    //       "phone": "+49 30 1007919",
    //       "score": 79,
    //       "sms_consent": false,
    //       "sms_consent_updated_at": "2026-11-05T16:00:00Z",
    //       "source": "form:launch-waitlist",
    //       "status": "SUBSCRIBED",
    //       "tags": [
    //         "trial"
    //       ],
    //       "timezone": "Europe/London",
    //       "tracking_optout": false,
    //       "updated_at": "2026-11-05T15:00:00Z"
    //     },
    //     {
    //       "attribution": {
    //         "utm_campaign": "oct-product-update",
    //         "utm_medium": "email",
    //         "utm_source": "newsletter"
    //       },
    //       "country_code": "ZA",
    //       "created_at": "2026-10-09T16:00:00Z",
    //       "custom_fields": {
    //         "industry": "Finance",
    //         "plan": "Business",
    //         "renewal_date": "2026-12-23",
    //         "seats": 11,
    //         "team_size": "51+"
    //       },
    //       "email": "lena.fischer@contoso.example",
    //       "first_name": "Lena",
    //       "id": "f25ace60-6582-5cc7-9226-f9367248776c",
    //       "job_title": "Marketing Lead",
    //       "last_activity_at": "2026-11-05T01:00:00Z",
    //       "last_name": "Fischer",
    //       "list_ids": [
    //         "b3e822bf-efcc-5f58-ba14-b485ee6d6542"
    //       ],
    //       "list_names": [
    //         "Product launch waitlist"
    //       ],
    //       "message_stats": {
    //         "hard_bounces": 0,
    //         "last_click_at": "2026-11-02T16:00:00Z",
    //         "last_open_at": "2026-11-05T04:00:00Z",
    //         "sent": 14,
    //         "soft_bounces": 0
    //       },
    //       "organisation_id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
    //       "organisation_name": "Contoso Ltd",
    //       "phone": "+49 30 1015838",
    //       "score": 84,
    //       "sms_consent": false,
    //       "sms_consent_updated_at": "2026-11-05T16:00:00Z",
    //       "source": "form:free-trial",
    //       "status": "SUBSCRIBED",
    //       "tags": [
    //         "waitlist"
    //       ],
    //       "timezone": "Africa/Johannesburg",
    //       "tracking_optout": false,
    //       "updated_at": "2026-11-05T14:00:00Z"
    //     },
    //     {
    //       "attribution": {
    //         "utm_campaign": "Utm campaign",
    //         "utm_medium": "none",
    //         "utm_source": "direct"
    //       },
    //       "country_code": "US",
    //       "created_at": "2026-09-28T16:00:00Z",
    //       "custom_fields": {
    //         "industry": "SaaS",
    //         "plan": "Free",
    //         "renewal_date": "2026-11-25",
    //         "seats": 14,
    //         "team_size": "1-10"
    //       },
    //       "email": "marcus.reid@adventureworks.example",
    //       "first_name": "Marcus",
    //       "id": "93f1db66-d1af-580b-b546-6a13f1774d12",
    //       "job_title": "CTO",
    //       "last_activity_at": "2026-11-04T18:00:00Z",
    //       "last_name": "Reid",
    //       "list_ids": [
    //         "09f5b2f9-2adc-5edd-a6fd-c20675680671"
    //       ],
    //       "list_names": [
    //         "Newsletter"
    //       ],
    //       "message_stats": {
    //         "hard_bounces": 0,
    //         "last_click_at": "2026-11-01T16:00:00Z",
    //         "last_open_at": "2026-11-04T23:00:00Z",
    //         "sent": 15,
    //         "soft_bounces": 0
    //       },
    //       "organisation_id": "3cc2755d-bcec-5a0c-afa0-71d35560b07d",
    //       "organisation_name": "Adventure Works",
    //       "phone": "+49 30 1023757",
    //       "score": 80,
    //       "sms_consent": false,
    //       "sms_consent_updated_at": "2026-11-05T16:00:00Z",
    //       "source": "import",
    //       "status": "SUBSCRIBED",
    //       "tags": [
    //         "churned"
    //       ],
    //       "timezone": "America/New_York",
    //       "tracking_optout": false,
    //       "updated_at": "2026-11-05T13:00:00Z"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "count": 412,
  "sample": [
    {
      "attribution": {
        "utm_campaign": "bf-2026",
        "utm_medium": "social",
        "utm_source": "linkedin"
      },
      "country_code": "GB",
      "created_at": "2026-10-20T16:00:00Z",
      "custom_fields": {
        "industry": "Retail",
        "plan": "Team",
        "renewal_date": "2026-12-14",
        "seats": 8,
        "team_size": "11-50"
      },
      "email": "tom.okafor@fabrikam.example",
      "first_name": "Tom",
      "id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "job_title": "Founder",
      "last_activity_at": "2026-11-05T08:00:00Z",
      "last_name": "Okafor",
      "list_ids": [
        "09f5b2f9-2adc-5edd-a6fd-c20675680671"
      ],
      "list_names": [
        "Newsletter"
      ],
      "message_stats": {
        "hard_bounces": 0,
        "last_click_at": "2026-11-03T16:00:00Z",
        "last_open_at": "2026-11-05T09:00:00Z",
        "sent": 13,
        "soft_bounces": 0
      },
      "organisation_id": "576a7630-a5b2-566c-ac00-caca59662cb0",
      "organisation_name": "Fabrikam Inc",
      "phone": "+49 30 1007919",
      "score": 79,
      "sms_consent": false,
      "sms_consent_updated_at": "2026-11-05T16:00:00Z",
      "source": "form:launch-waitlist",
      "status": "SUBSCRIBED",
      "tags": [
        "trial"
      ],
      "timezone": "Europe/London",
      "tracking_optout": false,
      "updated_at": "2026-11-05T15:00:00Z"
    },
    {
      "attribution": {
        "utm_campaign": "oct-product-update",
        "utm_medium": "email",
        "utm_source": "newsletter"
      },
      "country_code": "ZA",
      "created_at": "2026-10-09T16:00:00Z",
      "custom_fields": {
        "industry": "Finance",
        "plan": "Business",
        "renewal_date": "2026-12-23",
        "seats": 11,
        "team_size": "51+"
      },
      "email": "lena.fischer@contoso.example",
      "first_name": "Lena",
      "id": "f25ace60-6582-5cc7-9226-f9367248776c",
      "job_title": "Marketing Lead",
      "last_activity_at": "2026-11-05T01:00:00Z",
      "last_name": "Fischer",
      "list_ids": [
        "b3e822bf-efcc-5f58-ba14-b485ee6d6542"
      ],
      "list_names": [
        "Product launch waitlist"
      ],
      "message_stats": {
        "hard_bounces": 0,
        "last_click_at": "2026-11-02T16:00:00Z",
        "last_open_at": "2026-11-05T04:00:00Z",
        "sent": 14,
        "soft_bounces": 0
      },
      "organisation_id": "da09c739-6e09-5587-a658-88f6b9eea0bd",
      "organisation_name": "Contoso Ltd",
      "phone": "+49 30 1015838",
      "score": 84,
      "sms_consent": false,
      "sms_consent_updated_at": "2026-11-05T16:00:00Z",
      "source": "form:free-trial",
      "status": "SUBSCRIBED",
      "tags": [
        "waitlist"
      ],
      "timezone": "Africa/Johannesburg",
      "tracking_optout": false,
      "updated_at": "2026-11-05T14:00:00Z"
    },
    {
      "attribution": {
        "utm_campaign": "Utm campaign",
        "utm_medium": "none",
        "utm_source": "direct"
      },
      "country_code": "US",
      "created_at": "2026-09-28T16:00:00Z",
      "custom_fields": {
        "industry": "SaaS",
        "plan": "Free",
        "renewal_date": "2026-11-25",
        "seats": 14,
        "team_size": "1-10"
      },
      "email": "marcus.reid@adventureworks.example",
      "first_name": "Marcus",
      "id": "93f1db66-d1af-580b-b546-6a13f1774d12",
      "job_title": "CTO",
      "last_activity_at": "2026-11-04T18:00:00Z",
      "last_name": "Reid",
      "list_ids": [
        "09f5b2f9-2adc-5edd-a6fd-c20675680671"
      ],
      "list_names": [
        "Newsletter"
      ],
      "message_stats": {
        "hard_bounces": 0,
        "last_click_at": "2026-11-01T16:00:00Z",
        "last_open_at": "2026-11-04T23:00:00Z",
        "sent": 15,
        "soft_bounces": 0
      },
      "organisation_id": "3cc2755d-bcec-5a0c-afa0-71d35560b07d",
      "organisation_name": "Adventure Works",
      "phone": "+49 30 1023757",
      "score": 80,
      "sms_consent": false,
      "sms_consent_updated_at": "2026-11-05T16:00:00Z",
      "source": "import",
      "status": "SUBSCRIBED",
      "tags": [
        "churned"
      ],
      "timezone": "America/New_York",
      "tracking_optout": false,
      "updated_at": "2026-11-05T13:00:00Z"
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
        count: 412,sample: Some(vec![serde_json::from_value::<Contact>(serde_json::json!({"attribution":{"utm_campaign":"bf-2026","utm_medium":"social","utm_source":"linkedin"},"country_code":"GB","created_at":"2026-10-20T16:00:00Z","custom_fields":{"industry":"Retail","plan":"Team","renewal_date":"2026-12-14","seats":8,"team_size":"11-50"},"email":"tom.okafor@fabrikam.example","first_name":"Tom","id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","job_title":"Founder","last_activity_at":"2026-11-05T08:00:00Z","last_name":"Okafor","list_ids":["09f5b2f9-2adc-5edd-a6fd-c20675680671"],"list_names":["Newsletter"],"message_stats":{"hard_bounces":0,"last_click_at":"2026-11-03T16:00:00Z","last_open_at":"2026-11-05T09:00:00Z","sent":13,"soft_bounces":0},"organisation_id":"576a7630-a5b2-566c-ac00-caca59662cb0","organisation_name":"Fabrikam Inc","phone":"+49 30 1007919","score":79,"sms_consent":false,"sms_consent_updated_at":"2026-11-05T16:00:00Z","source":"form:launch-waitlist","status":"SUBSCRIBED","tags":["trial"],"timezone":"Europe/London","tracking_optout":false,"updated_at":"2026-11-05T15:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Contact>(serde_json::json!({"attribution":{"utm_campaign":"oct-product-update","utm_medium":"email","utm_source":"newsletter"},"country_code":"ZA","created_at":"2026-10-09T16:00:00Z","custom_fields":{"industry":"Finance","plan":"Business","renewal_date":"2026-12-23","seats":11,"team_size":"51+"},"email":"lena.fischer@contoso.example","first_name":"Lena","id":"f25ace60-6582-5cc7-9226-f9367248776c","job_title":"Marketing Lead","last_activity_at":"2026-11-05T01:00:00Z","last_name":"Fischer","list_ids":["b3e822bf-efcc-5f58-ba14-b485ee6d6542"],"list_names":["Product launch waitlist"],"message_stats":{"hard_bounces":0,"last_click_at":"2026-11-02T16:00:00Z","last_open_at":"2026-11-05T04:00:00Z","sent":14,"soft_bounces":0},"organisation_id":"da09c739-6e09-5587-a658-88f6b9eea0bd","organisation_name":"Contoso Ltd","phone":"+49 30 1015838","score":84,"sms_consent":false,"sms_consent_updated_at":"2026-11-05T16:00:00Z","source":"form:free-trial","status":"SUBSCRIBED","tags":["waitlist"],"timezone":"Africa/Johannesburg","tracking_optout":false,"updated_at":"2026-11-05T14:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Contact>(serde_json::json!({"attribution":{"utm_campaign":"Utm campaign","utm_medium":"none","utm_source":"direct"},"country_code":"US","created_at":"2026-09-28T16:00:00Z","custom_fields":{"industry":"SaaS","plan":"Free","renewal_date":"2026-11-25","seats":14,"team_size":"1-10"},"email":"marcus.reid@adventureworks.example","first_name":"Marcus","id":"93f1db66-d1af-580b-b546-6a13f1774d12","job_title":"CTO","last_activity_at":"2026-11-04T18:00:00Z","last_name":"Reid","list_ids":["09f5b2f9-2adc-5edd-a6fd-c20675680671"],"list_names":["Newsletter"],"message_stats":{"hard_bounces":0,"last_click_at":"2026-11-01T16:00:00Z","last_open_at":"2026-11-04T23:00:00Z","sent":15,"soft_bounces":0},"organisation_id":"3cc2755d-bcec-5a0c-afa0-71d35560b07d","organisation_name":"Adventure Works","phone":"+49 30 1023757","score":80,"sms_consent":false,"sms_consent_updated_at":"2026-11-05T16:00:00Z","source":"import","status":"SUBSCRIBED","tags":["churned"],"timezone":"America/New_York","tracking_optout":false,"updated_at":"2026-11-05T13:00:00Z"})).unwrap_or_default()]),
    })
}
