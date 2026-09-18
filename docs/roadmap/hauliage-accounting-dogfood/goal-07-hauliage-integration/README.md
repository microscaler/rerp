# Goal 7: Hauliage As The First Ordinary Consumer

## Objective

Prove that a live SaaS domain can delegate all accounting responsibility to RERP through public, tenant-aware APIs.

## Boundary

Hauliage owns:

- quotes and accepted commercial terms;
- shipper and haulier source identities;
- freight state and ePOD;
- disputes and operational adjustments;
- authoritative payment, escrow, and settlement events when those integrations exist;
- a durable accounting outbox.

RERP owns:

- counterparties and accounting-entity mappings;
- invoice and credit/debit note lifecycles;
- document numbers, tax snapshots, rounding, and rendering;
- journals, periods, accounts, posting, reversals, and audit;
- accounting status and financial records.

## Broad Outcomes

- A narrow typed RERP client using BRRTRouter HTTP and may_minihttp with rustls.
- A versioned consumer OpenAPI subset or generated SDK contract.
- Transactional outbox delivery with safe retries.
- Sesame service identity and tenant propagation.
- Commercial snapshots frozen at the qualifying Hauliage event.
- Hauliage UI reads authoritative RERP numbers, totals, status, and documents.
- Browser-side invoice calculations are removed as accounting authorities.

## Initial Acceptance Gates

- A qualifying ePOD/delivery event produces exactly one RERP invoice and balanced journal.
- RERP downtime loses no Hauliage accounting instruction.
- A retry cannot duplicate an invoice or journal.
- Cross-tenant requests and document reads fail.
- Hauliage cannot supply a different authenticated RERP tenant.
- UI and API values match the RERP accounting document.
- No RERP schema or operation is freight-specific.
- Cash or settlement entries occur only from authoritative external events, not quote acceptance or presentation-layer flags.

## Follow-On Dog-Food Slices

After the first customer invoice:

1. Haulier supplier invoice or self-billed document and settlement posting.
2. Approved dispute adjustment through credit/debit notes.
3. Payment allocation and escrow/cash liability entries.
4. Bank reconciliation.
5. Tax returns and statutory documents.
6. Treasury and cash forecasting.
7. Financial and management reporting.

These follow-on slices should deepen the same public platform rather than create a private Hauliage accounting subsystem.

## Questions To Thrash Out

- What exact event is the first invoice tax point?
- Is the marketplace principal or agent?
- Does the carrier invoice, or is self-billing required?
- Where is the authoritative commercial calculation frozen?
- Which RERP reads go directly from Hauliage and which are proxied by its BFF?
- What are the retry, dead-letter, replay, and operational-support workflows?

## Dependencies

- Goals 1 through 6.
- The Hauliage/RERP accounting-policy ADR.
