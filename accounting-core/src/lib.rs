//! Deterministic accounting invariants for RERP.
//!
//! This crate has no HTTP or database dependency. A runtime validates Sesame
//! identity, starts one Lifeguard RLS transaction, allocates numbers and IDs,
//! invokes this kernel, and atomically persists the resulting [`PostingPlan`].

mod error;
mod invoice;
mod trial_balance;
mod types;

pub use error::AccountingError;
pub use invoice::{
    credit_customer_invoice, post_customer_invoice, AccountingDocumentStatus,
    AccountingDocumentType, AuditAction, AuditEvent, CustomerInvoiceInstruction, InvoiceLineInput,
    InvoiceLineSnapshot, InvoiceSnapshot, JournalEntry, JournalLine, JournalSide, PostingPlan,
    TaxInput,
};
pub use trial_balance::{derive_trial_balance, TrialBalance, TrialBalanceAccount};
pub use types::{
    CurrencyCode, FiscalPeriodState, IdempotencyKey, PostingContext, PostingIdentifiers,
    RoundingPolicy, SourceReference,
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime};
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use std::convert::TryFrom;
    use uuid::Uuid;

    struct Fixture {
        tenant: String,
        legal_entity: Uuid,
        customer: Uuid,
        receivable: Uuid,
        revenue: Uuid,
        tax: Uuid,
        context: PostingContext,
        identifiers: PostingIdentifiers,
    }

    impl Fixture {
        fn new() -> Self {
            let tenant = "hauliage".to_owned();
            let legal_entity = Uuid::new_v4();
            let context = PostingContext {
                tenant_id: tenant.clone(),
                legal_entity_id: legal_entity,
                subject_id: Uuid::new_v4(),
                fiscal_period_id: Uuid::new_v4(),
                fiscal_period_start: date(2026, 7, 1),
                fiscal_period_end: date(2026, 7, 31),
                fiscal_period_state: FiscalPeriodState::Open,
                posted_at: datetime(2026, 7, 14, 9, 30, 0),
            };
            Self {
                tenant,
                legal_entity,
                customer: Uuid::new_v4(),
                receivable: Uuid::new_v4(),
                revenue: Uuid::new_v4(),
                tax: Uuid::new_v4(),
                context,
                identifiers: PostingIdentifiers {
                    invoice_id: Uuid::new_v4(),
                    journal_entry_id: Uuid::new_v4(),
                    invoice_number: "INV-2026-000001".to_owned(),
                    journal_entry_number: "JRN-2026-000001".to_owned(),
                },
            }
        }

        fn instruction(&self) -> CustomerInvoiceInstruction {
            CustomerInvoiceInstruction {
                idempotency_key: IdempotencyKey::try_from("delivery:123:invoice:v1")
                    .expect("valid key"),
                source: SourceReference {
                    system: "hauliage".to_owned(),
                    resource_type: "delivery".to_owned(),
                    resource_id: "123".to_owned(),
                },
                customer_id: self.customer,
                invoice_date: date(2026, 7, 14),
                due_date: date(2026, 8, 13),
                currency: CurrencyCode::try_from("GBP").expect("valid currency"),
                rounding: RoundingPolicy::new(2).expect("valid precision"),
                receivable_account_id: self.receivable,
                lines: vec![InvoiceLineInput {
                    description: "Freight service".to_owned(),
                    quantity: dec!(2),
                    unit_price: dec!(125.55),
                    discount_percent: dec!(10),
                    revenue_account_id: self.revenue,
                    tax: Some(TaxInput {
                        code: "VAT-20".to_owned(),
                        rate_percent: dec!(20),
                        liability_account_id: self.tax,
                    }),
                }],
            }
        }
    }

    #[test]
    fn posts_taxed_discounted_invoice_and_balanced_journal() {
        let fixture = Fixture::new();
        let plan = post_customer_invoice(
            &fixture.context,
            &fixture.identifiers,
            &fixture.instruction(),
        )
        .expect("valid posting");

        assert_eq!(plan.invoice.subtotal, dec!(251.10));
        assert_eq!(plan.invoice.discount_amount, dec!(25.11));
        assert_eq!(plan.invoice.tax_amount, dec!(45.20));
        assert_eq!(plan.invoice.total_amount, dec!(271.19));
        assert_eq!(plan.journal.total_debit, dec!(271.19));
        assert_eq!(plan.journal.total_credit, dec!(271.19));
        assert_eq!(plan.journal.lines.len(), 3);
        assert!(plan.journal.validate().is_ok());
    }

    #[test]
    fn rounds_half_away_from_zero_at_line_level() {
        let fixture = Fixture::new();
        let mut instruction = fixture.instruction();
        instruction.lines[0].quantity = dec!(1);
        instruction.lines[0].unit_price = dec!(1.005);
        instruction.lines[0].discount_percent = Decimal::ZERO;
        instruction.lines[0].tax = None;

        let plan = post_customer_invoice(&fixture.context, &fixture.identifiers, &instruction)
            .expect("valid posting");
        assert_eq!(plan.invoice.total_amount, dec!(1.01));
    }

    #[test]
    fn rejects_zero_quantity() {
        let fixture = Fixture::new();
        let mut instruction = fixture.instruction();
        instruction.lines[0].quantity = Decimal::ZERO;
        assert_eq!(
            post_customer_invoice(&fixture.context, &fixture.identifiers, &instruction),
            Err(AccountingError::NonPositiveQuantity(1))
        );
    }

    #[test]
    fn rejects_discount_above_one_hundred_percent() {
        let fixture = Fixture::new();
        let mut instruction = fixture.instruction();
        instruction.lines[0].discount_percent = dec!(100.01);
        assert_eq!(
            post_customer_invoice(&fixture.context, &fixture.identifiers, &instruction),
            Err(AccountingError::InvalidDiscount(1))
        );
    }

    #[test]
    fn rejects_closed_period() {
        let mut fixture = Fixture::new();
        fixture.context.fiscal_period_state = FiscalPeriodState::Closed;
        assert_eq!(
            post_customer_invoice(
                &fixture.context,
                &fixture.identifiers,
                &fixture.instruction()
            ),
            Err(AccountingError::ClosedPeriod)
        );
    }

    #[test]
    fn rejects_date_outside_period() {
        let fixture = Fixture::new();
        let mut instruction = fixture.instruction();
        instruction.invoice_date = date(2026, 8, 1);
        assert_eq!(
            post_customer_invoice(&fixture.context, &fixture.identifiers, &instruction),
            Err(AccountingError::DateOutsidePeriod)
        );
    }

    #[test]
    fn request_fingerprint_is_deterministic_and_payload_sensitive() {
        let fixture = Fixture::new();
        let first = post_customer_invoice(
            &fixture.context,
            &fixture.identifiers,
            &fixture.instruction(),
        )
        .expect("first");
        let second = post_customer_invoice(
            &fixture.context,
            &fixture.identifiers,
            &fixture.instruction(),
        )
        .expect("second");
        assert_eq!(first.request_fingerprint, second.request_fingerprint);

        let mut changed = fixture.instruction();
        changed.lines[0].unit_price = dec!(125.56);
        let third = post_customer_invoice(&fixture.context, &fixture.identifiers, &changed)
            .expect("changed");
        assert_ne!(first.request_fingerprint, third.request_fingerprint);
    }

    #[test]
    fn full_credit_note_reverses_original_journal() {
        let fixture = Fixture::new();
        let original = post_customer_invoice(
            &fixture.context,
            &fixture.identifiers,
            &fixture.instruction(),
        )
        .expect("original");
        let credit_ids = PostingIdentifiers {
            invoice_id: Uuid::new_v4(),
            journal_entry_id: Uuid::new_v4(),
            invoice_number: "CRN-2026-000001".to_owned(),
            journal_entry_number: "JRN-2026-000002".to_owned(),
        };
        let credit = credit_customer_invoice(
            &fixture.context,
            &credit_ids,
            IdempotencyKey::try_from("delivery:123:credit:v1").expect("key"),
            SourceReference {
                system: "hauliage".to_owned(),
                resource_type: "delivery_credit".to_owned(),
                resource_id: "123-credit".to_owned(),
            },
            &original,
            "Service cancelled",
        )
        .expect("credit");

        assert_eq!(
            credit.invoice.document_type,
            AccountingDocumentType::CustomerCreditNote
        );
        assert_eq!(
            credit.invoice.original_document_id,
            Some(original.invoice.id)
        );
        assert_eq!(
            credit.invoice.invoice_date,
            fixture.context.posted_at.date()
        );
        assert_eq!(credit.audit_event.occurred_at, fixture.context.posted_at);
        for (original_line, credit_line) in original.journal.lines.iter().zip(&credit.journal.lines)
        {
            assert_ne!(original_line.side, credit_line.side);
            assert_eq!(original_line.amount, credit_line.amount);
        }
        assert!(credit.journal.validate().is_ok());
    }

    #[test]
    fn credit_note_posts_in_a_later_open_period() {
        let fixture = Fixture::new();
        let original = post_customer_invoice(
            &fixture.context,
            &fixture.identifiers,
            &fixture.instruction(),
        )
        .expect("original");
        let mut august = fixture.context.clone();
        august.fiscal_period_id = Uuid::new_v4();
        august.fiscal_period_start = date(2026, 8, 1);
        august.fiscal_period_end = date(2026, 8, 31);
        august.posted_at = datetime(2026, 8, 4, 10, 0, 0);

        let credit = credit_customer_invoice(
            &august,
            &PostingIdentifiers {
                invoice_id: Uuid::new_v4(),
                journal_entry_id: Uuid::new_v4(),
                invoice_number: "CRN-2026-000002".to_owned(),
                journal_entry_number: "JRN-2026-000003".to_owned(),
            },
            IdempotencyKey::try_from("delivery:123:credit:v2").expect("key"),
            SourceReference {
                system: "hauliage".to_owned(),
                resource_type: "delivery_credit".to_owned(),
                resource_id: "123-credit-2".to_owned(),
            },
            &original,
            "August correction",
        )
        .expect("credit in later period");

        assert_eq!(credit.invoice.invoice_date, date(2026, 8, 4));
        assert_eq!(credit.invoice.fiscal_period_id, august.fiscal_period_id);
        assert_eq!(credit.journal.entry_date, date(2026, 8, 4));
    }

    #[test]
    fn trial_balance_nets_invoice_and_full_credit_to_zero() {
        let fixture = Fixture::new();
        let original = post_customer_invoice(
            &fixture.context,
            &fixture.identifiers,
            &fixture.instruction(),
        )
        .expect("original");
        let credit = credit_customer_invoice(
            &fixture.context,
            &PostingIdentifiers {
                invoice_id: Uuid::new_v4(),
                journal_entry_id: Uuid::new_v4(),
                invoice_number: "CRN-2026-000001".to_owned(),
                journal_entry_number: "JRN-2026-000002".to_owned(),
            },
            IdempotencyKey::try_from("delivery:123:credit:v1").expect("key"),
            SourceReference {
                system: "hauliage".to_owned(),
                resource_type: "delivery_credit".to_owned(),
                resource_id: "123-credit".to_owned(),
            },
            &original,
            "Cancelled",
        )
        .expect("credit");

        let balance = derive_trial_balance(
            &fixture.tenant,
            fixture.legal_entity,
            &[original.journal, credit.journal],
        )
        .expect("balance");
        assert_eq!(balance.total_debit, balance.total_credit);
        assert!(balance
            .accounts
            .iter()
            .all(|account| account.closing_debit.is_zero() && account.closing_credit.is_zero()));
    }

    #[test]
    fn trial_balance_rejects_cross_tenant_input() {
        let fixture = Fixture::new();
        let mut plan = post_customer_invoice(
            &fixture.context,
            &fixture.identifiers,
            &fixture.instruction(),
        )
        .expect("posting");
        plan.journal.tenant_id = "another-tenant".to_owned();
        assert_eq!(
            derive_trial_balance(&fixture.tenant, fixture.legal_entity, &[plan.journal]),
            Err(AccountingError::TenantMismatch)
        );
    }

    #[test]
    fn currency_and_idempotency_types_reject_ambiguous_values() {
        assert!(CurrencyCode::try_from("gbp").is_err());
        assert!(CurrencyCode::try_from("USDT").is_err());
        assert!(IdempotencyKey::try_from("").is_err());
        assert!(IdempotencyKey::try_from(" padded ").is_err());
    }

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("valid date")
    }

    fn datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        date(year, month, day)
            .and_hms_opt(hour, minute, second)
            .expect("valid datetime")
    }
}
