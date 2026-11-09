// User-owned controller for handler 'create_template'.

use crate::handlers::create_template::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BodyFormat;
#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::TemplateKind;

#[handler(CreateTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        body: "example".to_string(),
        body_format: Default::default(),
        channel: Default::default(),
        created_at: "example".to_string(),
        design: Some(Default::default()),
        id: "example".to_string(),
        is_archived: Some(true),
        kind: Default::default(),
        merge_fields: Some(vec![]),
        name: "example".to_string(),
        preheader: Some("example".to_string()),
        subject: Some("example".to_string()),
        text_body: Some("example".to_string()),
        thumbnail_url: Some("example".to_string()),
        updated_at: Some("example".to_string()),
        version: 42,
    })
}
