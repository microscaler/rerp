// User-owned controller for handler 'create_conversion_definition'.

use crate::handlers::create_conversion_definition::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConversionTrigger;

#[handler(CreateConversionDefinitionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        count_30d: Some(42),
        created_at: Some("example".to_string()),
        currency: Some("example".to_string()),
        id: "example".to_string(),
        last_at: Some("example".to_string()),
        name: "example".to_string(),
        once_per_contact: Some(true),
        revenue_30d: Some(rust_decimal::Decimal::new(12345, 2)),
        total: Some(42),
        trigger: Default::default(),
        value: Some(rust_decimal::Decimal::new(12345, 2)),
        value_mode: Some("example".to_string()),
    })
}
