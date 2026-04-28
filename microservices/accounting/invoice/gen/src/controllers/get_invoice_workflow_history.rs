// User-owned controller for handler 'get_invoice_workflow_history'.

use crate::handlers::get_invoice_workflow_history::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::InvoiceApprovalAction;

#[handler(GetInvoiceWorkflowHistoryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    }
}
