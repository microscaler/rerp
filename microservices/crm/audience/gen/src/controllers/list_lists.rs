// User-owned controller for handler 'list_lists'.

use crate::handlers::list_lists::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::MailingList;

#[handler(ListListsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "channel": "EMAIL",
    //       "confirmation_template_id": "be01905c-70da-59d9-b7ad-e59f4d2f4109",
    //       "created_at": "2026-03-10T16:00:00Z",
    //       "description": "Monthly product news",
    //       "double_opt_in": true,
    //       "growth_30d": [
    //         8,
    //         10,
    //         12
    //       ],
    //       "id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //       "is_archived": false,
    //       "name": "Newsletter",
    //       "pending_count": 144,
    //       "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
    //       "reminder_text": "You are receiving this because you signed up on our website.",
    //       "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
    //       "subscriber_count": 9870,
    //       "unsubscribed_count": 812
    //     },
    //     {
    //       "channel": "EMAIL",
    //       "confirmation_template_id": "cf79ecdf-e874-56bb-b2f4-00d88a332b24",
    //       "created_at": "2026-04-09T16:00:00Z",
    //       "description": "Account and billing updates",
    //       "double_opt_in": false,
    //       "growth_30d": [
    //         12,
    //         12,
    //         13
    //       ],
    //       "id": "f84269dd-7dba-5a82-ad63-986af03f39c1",
    //       "is_archived": false,
    //       "name": "Customers",
    //       "pending_count": 0,
    //       "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
    //       "reminder_text": "You are receiving this because you signed up on our website.",
    //       "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
    //       "subscriber_count": 1820,
    //       "unsubscribed_count": 41
    //     },
    //     {
    //       "channel": "EMAIL",
    //       "confirmation_template_id": "be01905c-70da-59d9-b7ad-e59f4d2f4109",
    //       "created_at": "2026-05-09T16:00:00Z",
    //       "description": "Invitations and recordings",
    //       "double_opt_in": true,
    //       "growth_30d": [
    //         6,
    //         9,
    //         7
    //       ],
    //       "id": "06b96791-3940-54b0-b719-717deb92aad3",
    //       "is_archived": false,
    //       "name": "Webinars",
    //       "pending_count": 22,
    //       "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
    //       "reminder_text": "You are receiving this because you signed up on our website.",
    //       "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
    //       "subscriber_count": 3210,
    //       "unsubscribed_count": 290
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 5
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "channel": "EMAIL",
      "confirmation_template_id": "be01905c-70da-59d9-b7ad-e59f4d2f4109",
      "created_at": "2026-03-10T16:00:00Z",
      "description": "Monthly product news",
      "double_opt_in": true,
      "growth_30d": [
        8,
        10,
        12
      ],
      "id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
      "is_archived": false,
      "name": "Newsletter",
      "pending_count": 144,
      "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
      "reminder_text": "You are receiving this because you signed up on our website.",
      "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
      "subscriber_count": 9870,
      "unsubscribed_count": 812
    },
    {
      "channel": "EMAIL",
      "confirmation_template_id": "cf79ecdf-e874-56bb-b2f4-00d88a332b24",
      "created_at": "2026-04-09T16:00:00Z",
      "description": "Account and billing updates",
      "double_opt_in": false,
      "growth_30d": [
        12,
        12,
        13
      ],
      "id": "f84269dd-7dba-5a82-ad63-986af03f39c1",
      "is_archived": false,
      "name": "Customers",
      "pending_count": 0,
      "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
      "reminder_text": "You are receiving this because you signed up on our website.",
      "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
      "subscriber_count": 1820,
      "unsubscribed_count": 41
    },
    {
      "channel": "EMAIL",
      "confirmation_template_id": "be01905c-70da-59d9-b7ad-e59f4d2f4109",
      "created_at": "2026-05-09T16:00:00Z",
      "description": "Invitations and recordings",
      "double_opt_in": true,
      "growth_30d": [
        6,
        9,
        7
      ],
      "id": "06b96791-3940-54b0-b719-717deb92aad3",
      "is_archived": false,
      "name": "Webinars",
      "pending_count": 22,
      "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
      "reminder_text": "You are receiving this because you signed up on our website.",
      "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
      "subscriber_count": 3210,
      "unsubscribed_count": 290
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 5
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<MailingList>(serde_json::json!({"channel":"EMAIL","confirmation_template_id":"be01905c-70da-59d9-b7ad-e59f4d2f4109","created_at":"2026-03-10T16:00:00Z","description":"Monthly product news","double_opt_in":true,"growth_30d":[8,10,12],"id":"09f5b2f9-2adc-5edd-a6fd-c20675680671","is_archived":false,"name":"Newsletter","pending_count":144,"postal_address":"Acme Co, 12 Example Street, 10115 Berlin, Germany","reminder_text":"You are receiving this because you signed up on our website.","sender_id":"37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2","subscriber_count":9870,"unsubscribed_count":812})).unwrap_or_default(), serde_json::from_value::<MailingList>(serde_json::json!({"channel":"EMAIL","confirmation_template_id":"cf79ecdf-e874-56bb-b2f4-00d88a332b24","created_at":"2026-04-09T16:00:00Z","description":"Account and billing updates","double_opt_in":false,"growth_30d":[12,12,13],"id":"f84269dd-7dba-5a82-ad63-986af03f39c1","is_archived":false,"name":"Customers","pending_count":0,"postal_address":"Acme Co, 12 Example Street, 10115 Berlin, Germany","reminder_text":"You are receiving this because you signed up on our website.","sender_id":"37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2","subscriber_count":1820,"unsubscribed_count":41})).unwrap_or_default(), serde_json::from_value::<MailingList>(serde_json::json!({"channel":"EMAIL","confirmation_template_id":"be01905c-70da-59d9-b7ad-e59f4d2f4109","created_at":"2026-05-09T16:00:00Z","description":"Invitations and recordings","double_opt_in":true,"growth_30d":[6,9,7],"id":"06b96791-3940-54b0-b719-717deb92aad3","is_archived":false,"name":"Webinars","pending_count":22,"postal_address":"Acme Co, 12 Example Street, 10115 Berlin, Germany","reminder_text":"You are receiving this because you signed up on our website.","sender_id":"37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2","subscriber_count":3210,"unsubscribed_count":290})).unwrap_or_default()],limit: 25,page: 1,total: 5,
    })
}
