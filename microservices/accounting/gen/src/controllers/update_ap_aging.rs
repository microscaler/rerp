
// User-owned controller for handler 'update_ap_aging'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::update_ap_aging::{ Request, Response };



#[handler(UpdateApAgingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    // Example response:
        // {
        //   "aging_date": "2024-01-31",
        //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
        //   "created_at": "2024-01-31T10:00:00Z",
        //   "currency_code": "USD",
        //   "current": 8000.0,
        //   "days_31_60": 0.0,
        //   "days_61_90": 0.0,
        //   "days_91_120": 0.0,
        //   "id": "a00b0e8400-e29b-41d4-a716-446655440000",
        //   "over_120": 0.0,
        //   "total_outstanding": 8000.0,
        //   "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
        // }
    match serde_json::from_str::<Response>(r###"{
  "aging_date": "2024-01-31",
  "company_id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2024-01-31T10:00:00Z",
  "currency_code": "USD",
  "current": 8000.0,
  "days_31_60": 0.0,
  "days_61_90": 0.0,
  "days_91_120": 0.0,
  "id": "a00b0e8400-e29b-41d4-a716-446655440000",
  "over_120": 0.0,
  "total_outstanding": 8000.0,
  "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
}"###) {
        Ok(parsed) => return parsed,
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }
    
    Response {
        
    }
    
    
}
