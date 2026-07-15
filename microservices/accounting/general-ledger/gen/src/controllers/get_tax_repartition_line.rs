// User-owned controller for handler 'get_tax_repartition_line'.

use crate::handlers::get_tax_repartition_line::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetTaxRepartitionLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
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
    })
}
