// User-owned controller for handler 'get_list_growth'.

use crate::handlers::get_list_growth::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetListGrowthController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "bounced": 42,
    //       "end": 9870,
    //       "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //       "list_name": "Newsletter",
    //       "start": 9601,
    //       "subscribed": 612,
    //       "top_source": "Website footer form",
    //       "unsubscribed": 301
    //     },
    //     {
    //       "bounced": 2,
    //       "end": 1820,
    //       "list_id": "f84269dd-7dba-5a82-ad63-986af03f39c1",
    //       "list_name": "Customers",
    //       "start": 1780,
    //       "subscribed": 58,
    //       "top_source": "Checkout",
    //       "unsubscribed": 16
    //     },
    //     {
    //       "bounced": 8,
    //       "end": 3210,
    //       "list_id": "06b96791-3940-54b0-b719-717deb92aad3",
    //       "list_name": "Webinars",
    //       "start": 2980,
    //       "subscribed": 418,
    //       "top_source": "Webinar registration",
    //       "unsubscribed": 180
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "bounced": 42,
      "end": 9870,
      "list_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
      "list_name": "Newsletter",
      "start": 9601,
      "subscribed": 612,
      "top_source": "Website footer form",
      "unsubscribed": 301
    },
    {
      "bounced": 2,
      "end": 1820,
      "list_id": "f84269dd-7dba-5a82-ad63-986af03f39c1",
      "list_name": "Customers",
      "start": 1780,
      "subscribed": 58,
      "top_source": "Checkout",
      "unsubscribed": 16
    },
    {
      "bounced": 8,
      "end": 3210,
      "list_id": "06b96791-3940-54b0-b719-717deb92aad3",
      "list_name": "Webinars",
      "start": 2980,
      "subscribed": 418,
      "top_source": "Webinar registration",
      "unsubscribed": 180
    }
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response(vec![serde_json::from_str::<serde_json::Value>(r#"{"bounced":42,"end":9870,"list_id":"09f5b2f9-2adc-5edd-a6fd-c20675680671","list_name":"Newsletter","start":9601,"subscribed":612,"top_source":"Website footer form","unsubscribed":301}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"bounced":2,"end":1820,"list_id":"f84269dd-7dba-5a82-ad63-986af03f39c1","list_name":"Customers","start":1780,"subscribed":58,"top_source":"Checkout","unsubscribed":16}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"bounced":8,"end":3210,"list_id":"06b96791-3940-54b0-b719-717deb92aad3","list_name":"Webinars","start":2980,"subscribed":418,"top_source":"Webinar registration","unsubscribed":180}"#).unwrap_or(serde_json::Value::Null)]))
}
