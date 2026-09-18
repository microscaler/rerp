//! General Ledger persistence registry.
//!
//! The first Accounting runtime uses the suite-owned foundation models under
//! `rerp_entities::accounting::foundation`. General Ledger deliberately owns no
//! second `accounts`, `journal_entries`, `journal_entry_lines`, or mutable
//! `account_balances` tables.

pub mod entity_registry {
    include!(concat!(env!("OUT_DIR"), "/entity_registry.rs"));
}
