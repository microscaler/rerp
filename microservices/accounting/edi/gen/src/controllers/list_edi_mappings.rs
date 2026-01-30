// User-owned controller for handler 'list_edi_mappings'.
use crate::handlers::list_edi_mappings::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EdiMapping;

#[handler(ListEdiMappingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: Some(42),
        page: Some(42),
        total: Some(42),
    }
}
