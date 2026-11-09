// User-owned controller for handler 'list_mailbox_messages'.

use crate::handlers::list_mailbox_messages::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::MailboxMessage;

#[handler(ListMailboxMessagesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "clicks": 0,
    //       "contact_ids": [
    //         "af512c4f-0beb-50b6-bf61-7a9d8bd417b5"
    //       ],
    //       "deal_ids": [
    //         "63a6661f-5860-50ed-8697-f28d29a3917f"
    //       ],
    //       "direction": "INBOUND",
    //       "from": "priya.natarajan@northwindtraders.example",
    //       "id": "8d5a1935-b54e-5567-ac56-e51e2656fa19",
    //       "mailbox_id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
    //       "opens": 0,
    //       "sent_at": "2026-11-05T11:00:00Z",
    //       "subject": "Re: Proposal for 120 seats",
    //       "to": [
    //         "alex@acme.example"
    //       ]
    //     },
    //     {
    //       "clicks": 1,
    //       "contact_ids": [
    //         "af512c4f-0beb-50b6-bf61-7a9d8bd417b5"
    //       ],
    //       "deal_ids": [
    //         "63a6661f-5860-50ed-8697-f28d29a3917f"
    //       ],
    //       "direction": "OUTBOUND",
    //       "from": "alex@acme.example",
    //       "id": "8e29079a-9ff4-5a6e-b030-f098c14b3148",
    //       "mailbox_id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
    //       "opens": 2,
    //       "sent_at": "2026-11-05T05:00:00Z",
    //       "subject": "Proposal for 120 seats",
    //       "to": [
    //         "priya.natarajan@northwindtraders.example"
    //       ]
    //     },
    //     {
    //       "clicks": 0,
    //       "contact_ids": [
    //         "7ed65e2d-0bd7-5e4e-9071-f4dac3456426"
    //       ],
    //       "deal_ids": [],
    //       "direction": "INBOUND",
    //       "from": "tom.okafor@fabrikam.example",
    //       "id": "7882e22e-5ff5-5c69-ac76-7fe91510e0b1",
    //       "mailbox_id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
    //       "opens": 0,
    //       "sent_at": "2026-11-05T06:00:00Z",
    //       "subject": "SSO docs",
    //       "to": [
    //         "alex@acme.example"
    //       ]
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 4
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "clicks": 0,
      "contact_ids": [
        "af512c4f-0beb-50b6-bf61-7a9d8bd417b5"
      ],
      "deal_ids": [
        "63a6661f-5860-50ed-8697-f28d29a3917f"
      ],
      "direction": "INBOUND",
      "from": "priya.natarajan@northwindtraders.example",
      "id": "8d5a1935-b54e-5567-ac56-e51e2656fa19",
      "mailbox_id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
      "opens": 0,
      "sent_at": "2026-11-05T11:00:00Z",
      "subject": "Re: Proposal for 120 seats",
      "to": [
        "alex@acme.example"
      ]
    },
    {
      "clicks": 1,
      "contact_ids": [
        "af512c4f-0beb-50b6-bf61-7a9d8bd417b5"
      ],
      "deal_ids": [
        "63a6661f-5860-50ed-8697-f28d29a3917f"
      ],
      "direction": "OUTBOUND",
      "from": "alex@acme.example",
      "id": "8e29079a-9ff4-5a6e-b030-f098c14b3148",
      "mailbox_id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
      "opens": 2,
      "sent_at": "2026-11-05T05:00:00Z",
      "subject": "Proposal for 120 seats",
      "to": [
        "priya.natarajan@northwindtraders.example"
      ]
    },
    {
      "clicks": 0,
      "contact_ids": [
        "7ed65e2d-0bd7-5e4e-9071-f4dac3456426"
      ],
      "deal_ids": [],
      "direction": "INBOUND",
      "from": "tom.okafor@fabrikam.example",
      "id": "7882e22e-5ff5-5c69-ac76-7fe91510e0b1",
      "mailbox_id": "22788e76-5b04-5e9c-97a4-49cade135ff3",
      "opens": 0,
      "sent_at": "2026-11-05T06:00:00Z",
      "subject": "SSO docs",
      "to": [
        "alex@acme.example"
      ]
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 4
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<MailboxMessage>(serde_json::json!({"clicks":0,"contact_ids":["af512c4f-0beb-50b6-bf61-7a9d8bd417b5"],"deal_ids":["63a6661f-5860-50ed-8697-f28d29a3917f"],"direction":"INBOUND","from":"priya.natarajan@northwindtraders.example","id":"8d5a1935-b54e-5567-ac56-e51e2656fa19","mailbox_id":"22788e76-5b04-5e9c-97a4-49cade135ff3","opens":0,"sent_at":"2026-11-05T11:00:00Z","subject":"Re: Proposal for 120 seats","to":["alex@acme.example"]})).unwrap_or_default(), serde_json::from_value::<MailboxMessage>(serde_json::json!({"clicks":1,"contact_ids":["af512c4f-0beb-50b6-bf61-7a9d8bd417b5"],"deal_ids":["63a6661f-5860-50ed-8697-f28d29a3917f"],"direction":"OUTBOUND","from":"alex@acme.example","id":"8e29079a-9ff4-5a6e-b030-f098c14b3148","mailbox_id":"22788e76-5b04-5e9c-97a4-49cade135ff3","opens":2,"sent_at":"2026-11-05T05:00:00Z","subject":"Proposal for 120 seats","to":["priya.natarajan@northwindtraders.example"]})).unwrap_or_default(), serde_json::from_value::<MailboxMessage>(serde_json::json!({"clicks":0,"contact_ids":["7ed65e2d-0bd7-5e4e-9071-f4dac3456426"],"deal_ids":[],"direction":"INBOUND","from":"tom.okafor@fabrikam.example","id":"7882e22e-5ff5-5c69-ac76-7fe91510e0b1","mailbox_id":"22788e76-5b04-5e9c-97a4-49cade135ff3","opens":0,"sent_at":"2026-11-05T06:00:00Z","subject":"SSO docs","to":["alex@acme.example"]})).unwrap_or_default()],limit: 20,page: 1,total: 4,
    })
}
