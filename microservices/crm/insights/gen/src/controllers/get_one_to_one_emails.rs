// User-owned controller for handler 'get_one_to_one_emails'.

use crate::handlers::get_one_to_one_emails::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::OneToOneEmailRow;

#[handler(GetOneToOneEmailsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "bounce_rate": 0.008,
    //       "bounces": 1,
    //       "email_name": "Quick question about your renewal",
    //       "sender_user_id": "alex",
    //       "sent": 131,
    //       "step_id": "8a1d086c-a04f-53cd-90b2-95cc18ad7a86",
    //       "unique_click_rate": 0.183,
    //       "unique_clicks": 24,
    //       "unique_open_rate": 0.679,
    //       "unique_opens": 89,
    //       "unique_replies": 17,
    //       "unique_reply_rate": 0.13,
    //       "workflow_id": "1b370767-8ad9-50d6-9a2f-1417e9310e08",
    //       "workflow_name": "Renewal reminder · 30 days out"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "bounce_rate": 0.008,
      "bounces": 1,
      "email_name": "Quick question about your renewal",
      "sender_user_id": "alex",
      "sent": 131,
      "step_id": "8a1d086c-a04f-53cd-90b2-95cc18ad7a86",
      "unique_click_rate": 0.183,
      "unique_clicks": 24,
      "unique_open_rate": 0.679,
      "unique_opens": 89,
      "unique_replies": 17,
      "unique_reply_rate": 0.13,
      "workflow_id": "1b370767-8ad9-50d6-9a2f-1417e9310e08",
      "workflow_name": "Renewal reminder · 30 days out"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 1
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<OneToOneEmailRow>(serde_json::json!({"bounce_rate":0.008,"bounces":1,"email_name":"Quick question about your renewal","sender_user_id":"alex","sent":131,"step_id":"8a1d086c-a04f-53cd-90b2-95cc18ad7a86","unique_click_rate":0.183,"unique_clicks":24,"unique_open_rate":0.679,"unique_opens":89,"unique_replies":17,"unique_reply_rate":0.13,"workflow_id":"1b370767-8ad9-50d6-9a2f-1417e9310e08","workflow_name":"Renewal reminder · 30 days out"})).unwrap_or_default()],limit: 20,page: 1,total: 1,
    })
}
