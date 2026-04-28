// User-owned controller for handler 'list_statutory_report_packs'.

use crate::handlers::list_statutory_report_packs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::StatutoryReportPack;

#[handler(ListStatutoryReportPacksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
