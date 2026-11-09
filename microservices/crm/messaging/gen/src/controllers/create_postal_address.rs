// User-owned controller for handler 'create_postal_address'.

use crate::handlers::create_postal_address::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreatePostalAddressController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
    //   "id": "9c7a38b3-2c83-5eb5-9d4c-7cf723835c99",
    //   "is_default": true,
    //   "name": "Berlin HQ",
    //   "used_by": [
    //     "Newsletter",
    //     "Customers",
    //     "Webinars"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
  "id": "9c7a38b3-2c83-5eb5-9d4c-7cf723835c99",
  "is_default": true,
  "name": "Berlin HQ",
  "used_by": [
    "Newsletter",
    "Customers",
    "Webinars"
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
        address: "Acme Co, 12 Example Street, 10115 Berlin, Germany".to_string(),
        id: "9c7a38b3-2c83-5eb5-9d4c-7cf723835c99".to_string(),
        is_default: Some(true),
        name: "Berlin HQ".to_string(),
        used_by: Some(vec![
            "Newsletter".to_string(),
            "Customers".to_string(),
            "Webinars".to_string(),
        ]),
    })
}
