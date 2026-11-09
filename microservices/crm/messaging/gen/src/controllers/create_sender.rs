// User-owned controller for handler 'create_sender'.

use crate::handlers::create_sender::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DnsRecord;
#[allow(unused_imports)]
use crate::handlers::types::Provider;
#[allow(unused_imports)]
use crate::handlers::types::SenderStatus;

#[handler(CreateSenderController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: "example".to_string(),
        dns_records: Some(vec![]),
        domain: "example".to_string(),
        from_address: "example".to_string(),
        from_name: "example".to_string(),
        id: "example".to_string(),
        last_checked_at: Some("example".to_string()),
        postal_address: Some("example".to_string()),
        provider: Some(Default::default()),
        reply_to: Some("example".to_string()),
        status: Default::default(),
    })
}
