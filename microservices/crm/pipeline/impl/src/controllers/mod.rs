// Implemented controllers for the just-enough CRM slice (leads + stages +
// summary). Every other operation in the pipeline spec stays unregistered
// until it has a real implementation — generated example handlers are
// forbidden in production (ADR 0001).

pub mod change_stage;
pub mod get_lead;
pub mod list_leads;
pub mod list_stages;
pub mod pipeline_summary;
pub mod update_lead;
