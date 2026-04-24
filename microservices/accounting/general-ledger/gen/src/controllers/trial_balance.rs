
// User-owned controller for handler 'trial_balance'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::trial_balance::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::TrialBalanceLine;



#[handler(TrialBalanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    
    
    Response {
        company_id: Some("example".to_string()),currency_code: Some("example".to_string()),difference: Some(3.14),lines: vec![],period_id: Some("example".to_string()),report_date: "example".to_string(),total_credits: 3.14,total_debits: 3.14,
    }
    
    
}
