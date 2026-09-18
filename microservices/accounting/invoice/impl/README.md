# Phase 1 Invoice Runtime

This executable is RERP's first public invoice-to-ledger vertical slice. It
accepts commercial facts, derives accounting scope from validated Sesame
claims and commits the invoice and balanced journal in one Lifeguard RLS
transaction.

The canonical source contract is
[`../../../../../openapi/accounting/invoice/openapi.yaml`](../../../../../openapi/accounting/invoice/openapi.yaml).
Five operations are active:
post a customer invoice, retrieve it, retrieve its journal, post a full credit
note, and materialize/retrieve its immutable PDF artifact. The executable
registers implementation controllers only;
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
| `RERP_OBJECT_STORE_ENDPOINT` | none | Internal S3-compatible endpoint |
| `RERP_OBJECT_STORE_PUBLIC_ENDPOINT` | object-store endpoint | Endpoint embedded in signed download URLs |
| `RERP_OBJECT_STORE_REGION` | `us-east-1` | SigV4 region |
| `RERP_OBJECT_STORE_BUCKET` | none | Private rendered-document bucket |
| `RERP_OBJECT_STORE_ACCESS_KEY` | none | Application-scoped access key |
| `RERP_OBJECT_STORE_SECRET_KEY` | none | Application-scoped secret key |
| `RERP_OBJECT_STORE_PRESIGN_SECONDS` | `300` | Signed URL lifetime, capped at 900 seconds |

Apply database resources in this order:

1. `microservices/accounting/sql/rls/v1/install.sql`;
2. `microservices/accounting/migrations/foundation/0001_generated_entities.sql`;
3. `microservices/accounting/migrations/foundation/0002_controls_and_rls.sql`;
4. `microservices/accounting/migrations/foundation/0003_document_artifacts.sql`.

`microservices/accounting/scripts/setup-db.sh` enforces that order and grants
the explicit Sesame RLS v1 function set plus table DML to the `rerp` role.
`microservices/accounting/scripts/setup-object-store.sh` provisions the private
MinIO bucket and least-privilege application credential used by Tilt.

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
- localization, settlement, partial credits and foreign currency.
