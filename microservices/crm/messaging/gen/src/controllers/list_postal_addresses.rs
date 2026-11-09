// User-owned controller for handler 'list_postal_addresses'.

use crate::handlers::list_postal_addresses::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PostalAddress;

#[handler(ListPostalAddressesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
    //       "id": "9c7a38b3-2c83-5eb5-9d4c-7cf723835c99",
    //       "is_default": true,
    //       "name": "Berlin HQ",
    //       "used_by": [
    //         "Newsletter",
    //         "Customers",
    //         "Webinars"
    //       ]
    //     },
    //     {
    //       "address": "Acme Co (Pty) Ltd, 4 Sample Road, Cape Town 8001, South Africa",
    //       "id": "0f860ccb-c518-552f-8ca2-767c52688164",
    //       "is_default": false,
    //       "name": "Cape Town office",
    //       "used_by": [
    //         "Partners"
    //       ]
    //     },
    //     {
    //       "address": "Acme Events, PO Box 123, 10115 Berlin, Germany",
    //       "id": "c699ea0a-3702-54fd-9422-9444801a3fa0",
    //       "is_default": false,
    //       "name": "Events",
    //       "used_by": []
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
      "id": "9c7a38b3-2c83-5eb5-9d4c-7cf723835c99",
      "is_default": true,
      "name": "Berlin HQ",
      "used_by": [
        "Newsletter",
        "Customers",
        "Webinars"
      ]
    },
    {
      "address": "Acme Co (Pty) Ltd, 4 Sample Road, Cape Town 8001, South Africa",
      "id": "0f860ccb-c518-552f-8ca2-767c52688164",
      "is_default": false,
      "name": "Cape Town office",
      "used_by": [
        "Partners"
      ]
    },
    {
      "address": "Acme Events, PO Box 123, 10115 Berlin, Germany",
      "id": "c699ea0a-3702-54fd-9422-9444801a3fa0",
      "is_default": false,
      "name": "Events",
      "used_by": []
    }
  ],
  "limit": 25,
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
        items: vec![serde_json::from_value::<PostalAddress>(serde_json::json!({"address":"Acme Co, 12 Example Street, 10115 Berlin, Germany","id":"9c7a38b3-2c83-5eb5-9d4c-7cf723835c99","is_default":true,"name":"Berlin HQ","used_by":["Newsletter","Customers","Webinars"]})).unwrap_or_default(), serde_json::from_value::<PostalAddress>(serde_json::json!({"address":"Acme Co (Pty) Ltd, 4 Sample Road, Cape Town 8001, South Africa","id":"0f860ccb-c518-552f-8ca2-767c52688164","is_default":false,"name":"Cape Town office","used_by":["Partners"]})).unwrap_or_default(), serde_json::from_value::<PostalAddress>(serde_json::json!({"address":"Acme Events, PO Box 123, 10115 Berlin, Germany","id":"c699ea0a-3702-54fd-9422-9444801a3fa0","is_default":false,"name":"Events","used_by":[]})).unwrap_or_default()],limit: 25,page: 1,total: 3,
    })
}
