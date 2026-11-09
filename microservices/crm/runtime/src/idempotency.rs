//! `Idempotency-Key` handling: claim the key with the request hash, replay the stored response for 24 hours, 409 `idempotency_mismatch` when the same key comes with a different body.
//!
//! Story: CRM-RT-03 (docs/crm/stories/).
