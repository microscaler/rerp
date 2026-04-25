
// User-owned controller for handler 'aging_summary'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::aging_summary::{ Request, Response };



#[handler(AgingSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        as_of_date: "example".to_string(),bad_debt_reserves: Some(3.14),bucket_1_30: 3.14,bucket_31_60: 3.14,bucket_61_90: 3.14,bucket_90_plus: 3.14,collection_efficiency_index: Some(3.14),company_id: "example".to_string(),currency_code: Some("example".to_string()),current: 3.14,days_sales_outstanding: Some(3.14),total_outstanding: 3.14,
    }
    
    
}
