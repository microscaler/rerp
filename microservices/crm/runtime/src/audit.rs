//! Writes one `audit.audit_entries` row per write, in the same transaction: service, entity, id, action, actor, changed fields before and after, request id (`US_55_39` AC-5).
//!
//! Story: CRM-RT-03 (docs/crm/stories/).
