// User-owned controller for handler 'update_asset'.

use crate::handlers::update_asset::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        accumulated_depreciation: Some(3.14),
        acquisition_cost: 3.14,
        acquisition_date: "example".to_string(),
        acquisition_gl_entry_id: Some("example".to_string()),
        asset_number: "example".to_string(),
        category_id: "example".to_string(),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        custodian_id: Some("example".to_string()),
        department_id: Some("example".to_string()),
        description: "example".to_string(),
        id: "example".to_string(),
        insurance_policy: Some("example".to_string()),
        location: "example".to_string(),
        name: "example".to_string(),
        net_book_value: Some(3.14),
        notes: Some("example".to_string()),
        serial_number: Some("example".to_string()),
        status: "example".to_string(),
        supplier_id: Some("example".to_string()),
        updated_at: Some("example".to_string()),
        useful_life_months: Some(42),
        warranty_expiry: Some("example".to_string()),
    })
}
