// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::list_leads::Request;
use rerp_crm_pipeline_gen::handlers::types::Lead;
use serde_json::{json, Value};

fn matches_search(lead: &Lead, needle: &str) -> bool {
    let n = needle.to_lowercase();
    lead.name.to_lowercase().contains(&n)
        || lead
            .email_normalized
            .as_deref()
            .is_some_and(|e| e.contains(&n))
        || lead
            .company_name
            .as_deref()
            .is_some_and(|c| c.to_lowercase().contains(&n))
}

#[handler(ListLeadsController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_viewer(req.jwt_claims.as_ref()) {
        return denied;
    }
    let leads = match crate::supabase::fetch_leads() {
        Ok(leads) => leads,
        Err(error) => {
            return HttpJson::new(502, json!({ "code": 502, "message": error }));
        }
    };

    let data = req.data;
    let filtered: Vec<&Lead> = leads
        .iter()
        .filter(|lead| match data.search.as_deref() {
            Some(s) if !s.is_empty() => matches_search(lead, s),
            _ => true,
        })
        .filter(|lead| match data.filter_stage_id.as_deref() {
            Some(id) if !id.is_empty() => lead.stage_id.as_deref() == Some(id),
            _ => true,
        })
        .filter(|lead| match data.filter_won_status.as_deref() {
            Some(ws) if !ws.is_empty() => lead.won_status.as_deref() == Some(ws),
            _ => true,
        })
        .collect();

    let total = filtered.len();
    let page = data.page.unwrap_or(1).max(1);
    let limit = data.limit.unwrap_or(50).clamp(1, 100);
    let start = usize::try_from((page - 1) * limit).unwrap_or(0);
    let items: Vec<&Lead> = filtered
        .into_iter()
        .skip(start)
        .take(usize::try_from(limit).unwrap_or(0))
        .collect();

    HttpJson::new(
        200,
        json!({ "items": items, "total": total, "page": page, "limit": limit }),
    )
}
