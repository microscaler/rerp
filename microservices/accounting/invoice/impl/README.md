# Phase 1 Invoice Runtime

This executable is RERP's first public invoice-to-ledger vertical slice. It
accepts commercial facts, derives accounting scope from validated Sesame
claims and commits the invoice and balanced journal in one Lifeguard RLS
transaction.

The source contract is
[`../openapi/phase1.yaml`](../openapi/phase1.yaml). Only four operations are
active: post a customer invoice, retrieve it, retrieve its journal and post a
full credit note. The executable registers implementation controllers only;
generated example controllers are not a runtime fallback.

## Runtime boundary

1. BRRTRouter validates the bearer token and supplies validated claims.
2. `identity.rs` builds a complete `SessionContext` and rejects missing or
   inconsistent tenant, subject, organization or session claims.
3. `http_support.rs` opens `LifeguardPool::with_session_transaction`.
4. `posting.rs` resolves the active legal entity, open fiscal period and
   `ACCOUNTS_RECEIVABLE`, `REVENUE` and optional `TAX_PAYABLE` control accounts.
5. `rerp-accounting-core` calculates immutable document and journal facts.
6. Typed Lifeguard records persist all facts, the audit event and completed
   idempotency record before commit.

Request bodies cannot select tenant, legal entity, fiscal period or GL account.
Phase 1 permits base-currency posting only and supports full credit notes only.

## Configuration

| Variable | Default | Meaning |
|---|---|---|
| `DB_HOST` | `postgres.data.svc.cluster.local` in Kubernetes, otherwise `localhost` | PostgreSQL host |
| `DB_PORT` | `5432` | PostgreSQL port |
| `DB_USER` | `rerp` | Non-superuser runtime role |
| `DB_PASS` | `RERP_DB_PASSWORD`, then empty | Runtime role password |
| `DB_NAME` | `rerp` | Database name |
| `DB_POOL_MAX` | `10` | Primary Lifeguard worker slots |

Apply database resources in this order:

1. `sql/rls/v1/install.sql`;
2. `migrations/accounting/foundation/0001_generated_entities.sql`;
3. `migrations/accounting/foundation/0002_controls_and_rls.sql`.

`scripts/setup-db.sh` enforces that order and grants the explicit Sesame RLS v1
function set plus table DML to the `rerp` role.

## Verification

Ordinary tests and lint:

```bash
cd microservices
cargo test -p rerp_accounting_invoice
cargo clippy -p rerp_accounting_invoice --all-targets --no-deps -- -D warnings
```

The ignored test `live_post_retry_conflict_retrieve_and_credit` requires a
disposable migrated and seeded PostgreSQL database. It proves posting, balanced
journal, idempotent retry, changed-payload conflict, retrieval and full credit
through the same transaction adapter used by controllers. Never run it against
a persistent environment.

## Explicit remaining work

- HTTPS execution through the generated consumer client;
- immutable rendered-document storage and retrieval;
- localization, settlement, partial credits and foreign currency.
