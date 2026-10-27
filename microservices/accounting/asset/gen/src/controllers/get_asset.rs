// User-owned controller for handler 'get_asset'.

use crate::handlers::get_asset::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DepreciationEntry;
#[allow(unused_imports)]
use crate::handlers::types::RevaluationRecord;

#[handler(GetAssetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        accumulated_depreciation: Some(1.5),
        acquisition_cost: 1.5,
        acquisition_date: "example".to_string(),
        acquisition_gl_entry_id: Some("example".to_string()),
        asset_number: "example".to_string(),
        category_id: "example".to_string(),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        custodian_id: Some("example".to_string()),
        department_id: Some("example".to_string()),
        depreciation_schedule: Some(vec![]),
        description: Some("example".to_string()),
        disposal_record: Some(Default::default()),
        id: "example".to_string(),
        insurance_policy: Some("example".to_string()),
        location: Some("example".to_string()),
        name: "example".to_string(),
        net_book_value: Some(1.5),
        notes: Some("example".to_string()),
        revaluation_history: Some(vec![]),
        serial_number: Some("example".to_string()),
        status: "example".to_string(),
        supplier_id: Some("example".to_string()),
        updated_at: Some("example".to_string()),
        useful_life_months: Some(42),
        warranty_expiry: Some("example".to_string()),
    })
}
