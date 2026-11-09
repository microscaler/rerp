// User-owned controller for handler 'list_contacts'.

use crate::handlers::list_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Contact;

#[handler(ListContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "attribution": {
    //         "utm_campaign": "q4-webinars",
    //         "utm_medium": "cpc",
    //         "utm_source": "google"
    //       },
    //       "country_code": "DE",
    //       "created_at": "2026-10-31T16:00:00Z",
    //       "custom_fields": {
    //         "industry": "Logistics",
    //         "plan": "Free",
    //         "renewal_date": "2026-11-25",
    //         "seats": 5,
    //         "team_size": "1-10"
    //       },
    //       "email": "priya.natarajan@northwindtraders.example",
    //       "first_name": "Priya",
    //       "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "job_title": "Head of Operations",
    //       "last_activity_at": "2026-11-05T15:00:00Z",
    //       "last_name": "Natarajan",
    //       "list_ids": [
    //         "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //         "f84269dd-7dba-5a82-ad63-986af03f39c1"
    //       ],
    //       "list_names": [
    //         "Newsletter",
    //         "Customers"
    //       ],
    //       "message_stats": {
    //         "hard_bounces": 0,
    //         "last_click_at": "2026-11-04T16:00:00Z",
    //         "last_open_at": "2026-11-05T14:00:00Z",
    //         "sent": 12,
    //         "soft_bounces": 0
    //       },
    //       "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
    //       "organisation_name": "Northwind Traders",
    //       "phone": "+49 30 1000000",
    //       "score": 85,
    //       "sms_consent": true,
    //       "sms_consent_updated_at": "2026-09-26T16:00:00Z",
    //       "source": "form:website-footer",
    //       "status": "SUBSCRIBED",
    //       "tags": [
    //         "customer",
    //         "enterprise"
    //       ],
    //       "timezone": "Europe/Berlin",
    //       "tracking_optout": false,
    //       "updated_at": "2026-11-05T16:00:00Z"
    //     },
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
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 30
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "attribution": {
        "utm_campaign": "q4-webinars",
        "utm_medium": "cpc",
        "utm_source": "google"
      },
      "country_code": "DE",
      "created_at": "2026-10-31T16:00:00Z",
      "custom_fields": {
        "industry": "Logistics",
        "plan": "Free",
        "renewal_date": "2026-11-25",
        "seats": 5,
        "team_size": "1-10"
      },
      "email": "priya.natarajan@northwindtraders.example",
      "first_name": "Priya",
      "id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "job_title": "Head of Operations",
      "last_activity_at": "2026-11-05T15:00:00Z",
      "last_name": "Natarajan",
      "list_ids": [
        "09f5b2f9-2adc-5edd-a6fd-c20675680671",
        "f84269dd-7dba-5a82-ad63-986af03f39c1"
      ],
      "list_names": [
        "Newsletter",
        "Customers"
      ],
      "message_stats": {
        "hard_bounces": 0,
        "last_click_at": "2026-11-04T16:00:00Z",
        "last_open_at": "2026-11-05T14:00:00Z",
        "sent": 12,
        "soft_bounces": 0
      },
      "organisation_id": "d7a2e2b8-61b1-5687-8aee-0ead8943ba3b",
      "organisation_name": "Northwind Traders",
      "phone": "+49 30 1000000",
      "score": 85,
      "sms_consent": true,
      "sms_consent_updated_at": "2026-09-26T16:00:00Z",
      "source": "form:website-footer",
      "status": "SUBSCRIBED",
      "tags": [
        "customer",
        "enterprise"
      ],
      "timezone": "Europe/Berlin",
      "tracking_optout": false,
      "updated_at": "2026-11-05T16:00:00Z"
    },
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
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 30
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Contact>(serde_json::json!({"attribution":{"utm_campaign":"q4-webinars","utm_medium":"cpc","utm_source":"google"},"country_code":"DE","created_at":"2026-10-31T16:00:00Z","custom_fields":{"industry":"Logistics","plan":"Free","renewal_date":"2026-11-25","seats":5,"team_size":"1-10"},"email":"priya.natarajan@northwindtraders.example","first_name":"Priya","id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","job_title":"Head of Operations","last_activity_at":"2026-11-05T15:00:00Z","last_name":"Natarajan","list_ids":["09f5b2f9-2adc-5edd-a6fd-c20675680671","f84269dd-7dba-5a82-ad63-986af03f39c1"],"list_names":["Newsletter","Customers"],"message_stats":{"hard_bounces":0,"last_click_at":"2026-11-04T16:00:00Z","last_open_at":"2026-11-05T14:00:00Z","sent":12,"soft_bounces":0},"organisation_id":"d7a2e2b8-61b1-5687-8aee-0ead8943ba3b","organisation_name":"Northwind Traders","phone":"+49 30 1000000","score":85,"sms_consent":true,"sms_consent_updated_at":"2026-09-26T16:00:00Z","source":"form:website-footer","status":"SUBSCRIBED","tags":["customer","enterprise"],"timezone":"Europe/Berlin","tracking_optout":false,"updated_at":"2026-11-05T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Contact>(serde_json::json!({"attribution":{"utm_campaign":"bf-2026","utm_medium":"social","utm_source":"linkedin"},"country_code":"GB","created_at":"2026-10-20T16:00:00Z","custom_fields":{"industry":"Retail","plan":"Team","renewal_date":"2026-12-14","seats":8,"team_size":"11-50"},"email":"tom.okafor@fabrikam.example","first_name":"Tom","id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","job_title":"Founder","last_activity_at":"2026-11-05T08:00:00Z","last_name":"Okafor","list_ids":["09f5b2f9-2adc-5edd-a6fd-c20675680671"],"list_names":["Newsletter"],"message_stats":{"hard_bounces":0,"last_click_at":"2026-11-03T16:00:00Z","last_open_at":"2026-11-05T09:00:00Z","sent":13,"soft_bounces":0},"organisation_id":"576a7630-a5b2-566c-ac00-caca59662cb0","organisation_name":"Fabrikam Inc","phone":"+49 30 1007919","score":79,"sms_consent":false,"sms_consent_updated_at":"2026-11-05T16:00:00Z","source":"form:launch-waitlist","status":"SUBSCRIBED","tags":["trial"],"timezone":"Europe/London","tracking_optout":false,"updated_at":"2026-11-05T15:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Contact>(serde_json::json!({"attribution":{"utm_campaign":"oct-product-update","utm_medium":"email","utm_source":"newsletter"},"country_code":"ZA","created_at":"2026-10-09T16:00:00Z","custom_fields":{"industry":"Finance","plan":"Business","renewal_date":"2026-12-23","seats":11,"team_size":"51+"},"email":"lena.fischer@contoso.example","first_name":"Lena","id":"f25ace60-6582-5cc7-9226-f9367248776c","job_title":"Marketing Lead","last_activity_at":"2026-11-05T01:00:00Z","last_name":"Fischer","list_ids":["b3e822bf-efcc-5f58-ba14-b485ee6d6542"],"list_names":["Product launch waitlist"],"message_stats":{"hard_bounces":0,"last_click_at":"2026-11-02T16:00:00Z","last_open_at":"2026-11-05T04:00:00Z","sent":14,"soft_bounces":0},"organisation_id":"da09c739-6e09-5587-a658-88f6b9eea0bd","organisation_name":"Contoso Ltd","phone":"+49 30 1015838","score":84,"sms_consent":false,"sms_consent_updated_at":"2026-11-05T16:00:00Z","source":"form:free-trial","status":"SUBSCRIBED","tags":["waitlist"],"timezone":"Africa/Johannesburg","tracking_optout":false,"updated_at":"2026-11-05T14:00:00Z"})).unwrap_or_default()],limit: 25,page: 1,total: 30,
    })
}
