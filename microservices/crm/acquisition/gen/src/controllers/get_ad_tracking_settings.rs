// User-owned controller for handler 'get_ad_tracking_settings'.

use crate::handlers::get_ad_tracking_settings::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdTrackingSettingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        capture_referrer: Some(true),
        capture_utm: Some(true),
        click_id_params: Some(vec![]),
        consent: Some(Default::default()),
        cookie: Some(Default::default()),
        default_attribution_model: Some("example".to_string()),
        default_window_days: Some(42),
        fingerprint: Some(Default::default()),
        identify_on: Some(vec![]),
        link_decoration: Some(Default::default()),
        own_click_id_param: Some("example".to_string()),
        updated_at: Some("example".to_string()),
    })
}
