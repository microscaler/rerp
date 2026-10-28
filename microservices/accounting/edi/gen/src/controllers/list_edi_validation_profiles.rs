// User-owned controller for handler 'list_edi_validation_profiles'.

use crate::handlers::list_edi_validation_profiles::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EdiValidationProfile;

#[handler(ListEdiValidationProfilesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
