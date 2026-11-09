// User-owned controller for handler 'get_contact_journey'.

use crate::handlers::get_contact_journey::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdConversion;
#[allow(unused_imports)]
use crate::handlers::types::AdTouchpoint;

#[handler(GetContactJourneyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        contact_id: Some("example".to_string()),
        contact_name: Some("example".to_string()),
        conversions: Some(vec![]),
        devices: Some(42),
        first_touch: Some(Default::default()),
        identifiers: Some(Default::default()),
        last_touch: Some(Default::default()),
        model: Some("example".to_string()),
        revenue: Some(rust_decimal::Decimal::new(12345, 2)),
        touchpoints: Some(vec![]),
    })
}
