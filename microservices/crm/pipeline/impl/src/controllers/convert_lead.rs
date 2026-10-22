// BRRTRouter: user-owned
//! Convert a lead (ADR 003): publish a signed conversion to RERP, then mark the lead converted in the funnel.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_pipeline_gen::handlers::convert_lead::Request;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::conversion::{self, PublishError};

fn fail(status: u16, message: &str) -> HttpJson<Value> {
    HttpJson::new(status, json!({ "code": status, "message": message }))
}

#[handler(ConvertLeadController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_editor(req.jwt_claims.as_ref()) {
        return denied;
    }
    let staff = match conversion::staff(req.jwt_claims.as_ref()) {
        Ok(staff) => staff,
        Err(message) => return fail(403, &message),
    };
    let data = req.data;
    let Ok(lead) = Uuid::parse_str(&data.id) else {
        return fail(400, "lead id must be a UUID");
    };
    let Some(reason) = conversion::reason(&data.reason) else {
        return fail(
            400,
            "reason must be order or contract; paid Stripe checkouts convert automatically",
        );
    };
    let existing = match crate::supabase::fetch_lead(&data.id) {
        Ok(Some(found)) => found,
        Ok(None) => return fail(404, "lead not found"),
        Err(error) => return fail(502, &error),
    };

    let conv = conversion::build(lead, &staff, reason, &data.reference, &data.customer);
    let message_id = match conversion::publish(&conv) {
        Ok(id) => id,
        Err(PublishError::Invalid(message)) => return fail(400, &message),
        Err(PublishError::Unavailable(message)) => {
            eprintln!("convert_lead {lead}: not published: {message}");
            return fail(
                503,
                "the conversion could not be published; nothing was changed, try again",
            );
        }
    };
    // The note is the lead's triage text: keep what staff wrote and add the conversion beneath it.
    let line = format!(
        "Converted ({} {}): crossed to RERP as conversion {}",
        reason.as_str(),
        conv.reference,
        conv.conversion_id
    );
    let note = match existing
        .description
        .as_deref()
        .map(str::trim)
        .filter(|d| !d.is_empty())
    {
        Some(previous) if previous.contains(&line) => previous.to_string(),
        Some(previous) => format!("{previous}\n\n{line}"),
        None => line,
    };
    if let Err(error) = crate::supabase::write_state(&data.id, Some("converted"), Some(&note)) {
        // Published but not marked: converting again republishes the same conversion, which RERP ignores.
        eprintln!("convert_lead {lead}: published {message_id} but marking failed: {error}");
        return fail(502, "the conversion was published but the lead could not be marked; convert again to finish");
    }
    HttpJson::new(
        202,
        json!({
            "lead_id": lead,
            "conversion_id": conv.conversion_id,
            "reason": reason.as_str(),
            "stage": "converted",
            "message_id": message_id,
        }),
    )
}
