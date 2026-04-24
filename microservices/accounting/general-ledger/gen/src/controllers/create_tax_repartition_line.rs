// User-owned controller for handler 'create_tax_repartition_line'.

use crate::handlers::create_tax_repartition_line::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTaxRepartitionLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        account_code: Some("example".to_string()),
        account_id: "example".to_string(),
        account_name: Some("example".to_string()),
        created_at: Some("example".to_string()),
        id: "example".to_string(),
        percentage: 3.14,
        ratio: 3.14,
        repartition_type: "example".to_string(),
        tag_ids: Some(vec![]),
        tax_id: "example".to_string(),
        tax_name: Some("example".to_string()),
        updated_at: Some("example".to_string()),
    }
}
