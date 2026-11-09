// User-owned controller for handler 'get_ad_buy_delivery'.

use crate::handlers::get_ad_buy_delivery::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdBuyDeliveryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        buy_id: Some("example".to_string()),
        days: Some(vec![]),
    })
}
