// BRRTRouter: user-owned
//
// Funnel counts for the portal's header tiles: totals per stage plus the
// honest revenue view (annualised plan interest, probability-weighted by
// stage). "This week" style slicing happens client-side from create_date.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::pipeline_summary::Request;
use serde_json::{json, Value};

#[handler(PipelineSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let leads = match crate::supabase::fetch_leads() {
        Ok(leads) => leads,
        Err(error) => {
            return HttpJson::new(502, json!({ "code": 502, "message": error }));
        }
    };

    let mut total_revenue = 0.0f64;
    let mut total_weighted = 0.0f64;
    let stages: Vec<Value> = crate::supabase::STAGES
        .iter()
        .map(|def| {
            let in_stage: Vec<_> = leads
                .iter()
                .filter(|l| l.stage_id.as_deref() == Some(def.id))
                .collect();
            let revenue: f64 = in_stage.iter().filter_map(|l| l.expected_revenue).sum();
            let weighted = revenue * (def.probability as f64) / 100.0;
            total_revenue += revenue;
            total_weighted += weighted;
            json!({
                "stage_id": def.id,
                "stage_name": def.name,
                "count": in_stage.len(),
                "revenue": revenue,
                "weighted_revenue": weighted,
                "probability": def.probability,
            })
        })
        .collect();

    HttpJson::new(
        200,
        json!({
            "total_opportunities": leads.len(),
            "total_revenue": total_revenue,
            "total_weighted_revenue": total_weighted,
            "stages": stages,
        }),
    )
}
