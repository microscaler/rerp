// User-owned controller for handler 'list_edi_acknowledgments'.

use crate::handlers::list_edi_acknowledgments::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EdiAcknowledgment;

#[handler(ListEdiAcknowledgmentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
