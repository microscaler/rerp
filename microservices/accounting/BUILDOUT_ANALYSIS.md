# Accounting Suite — Build-Out Analysis and Story Backlog

- **Date**: 2026-10-28
- **Audience**: engineers starting feature work on the Accounting suite
- **Authority**: gap map lives in
  [`docs/roadmap/hauliage-accounting-dogfood/service-readiness-plan/README.md`](../../docs/roadmap/hauliage-accounting-dogfood/service-readiness-plan/README.md)
  (WP0–WP8). This document is the *current-state delta* against that plan plus
  an executable story backlog. Where the two disagree, the roadmap plan's
  acceptance criteria win; this file wins on freshness.
- **Method**: every state claim below was read from this tree at `main`
  (post #109 merge), not from wiki memory. Commands to reproduce are in the
  appendix.

---

## 1. Executive summary

The suite's **foundation is real, two services are real, and the rest is a
convincing facade**. Invoice (3,274 LOC impl: posting, period/control checks,
idempotency, numbering, PDF, MinIO artifacts) and General Ledger (1,683 LOC,
four honest read routes over the 10-table `accounting/foundation`) carry
delivered behavior with live non-superuser RLS acceptance. AR, AP, bank-sync,
and financial-reports are regenerated TODO stubs that **return fabricated
example values** (`bucket_1_30: 1.5`, `"example"`) and additionally dispatch
generated mock handlers via `register_from_spec`. The BFF is an honest 155-op
proxy standing in front of mostly-fake upstreams.

Since the roadmap plan was written (2026-07-15/16), the biggest change is
hygiene: after PR #109 and the #110 dependency rollup, **every accounting crate
compiles in `default-members` and CI gates are green** — the "780 errors"
blocker the wiki records is closed. What remains is not build problems; it is
the plan's core warning, verbatim: *"Do not mark a service ready merely because
it compiles, has an OpenAPI contract, or returns a generated example
response."*

The critical path to building out is: **runtime truthfulness → contract
narrowing → GL mutation breadth → first subledger vertical (AR) → AP →
banking (mock connector) → reports → BFF gating**. Bank production adapters
stay externally blocked until a real institution agreement exists.

---

## 2. Current state (verified 2026-10-28)

### 2.1 Per-service reality

| Service | Impl LOC | Controllers | Reality | Evidence |
|---|---:|---:|---|---|
| `invoice` | 3,274 | 7 | **Real.** posting.rs, pdf.rs, object_store.rs, document.rs, identity.rs; 5 honest routes; decimal-string contract; live acceptance suite | `invoice/impl/src/` |
| `general-ledger` | 1,683 | 5 | **Real but narrow.** 4 read-only ops (accounts, periods, journal retrieval, as-of single-currency trial balance); `accounting:ledger:read` | `general-ledger/impl/src/ledger.rs` |
| `accounts-receivable` | 2,036 | 25 | **Fake.** Every controller is a TODO stub returning example data; `main.rs:316` registers `register_from_spec` mock dispatch | `accounts-receivable/impl/src/controllers/aging_summary.rs` |
| `accounts-payable` | 1,856 | 21 | **Fake.** Same pattern | `accounts-payable/impl/src/` |
| `bank-sync` | 2,277 | 25 | **Fake.** Same pattern | `bank-sync/impl/src/` |
| `financial-reports` | 1,776 | 19 | **Fake.** Same pattern | `financial-reports/impl/src/` |
| `bff` | 654 | 1 (registry) | **Honest proxy**, 155 ops, all `x-brrtrouter-impl: false`; upstreams mostly fake | `bff/impl/src/main.rs` |

Stub example: `aging_summary` returns `bucket_1_30: 1.5` with
`company_id: "example"` — note the `f64` buckets come from the **spec**,
violating constraint #10 (no binary floats for money) at the contract layer.

### 2.2 Foundation and data layer

- 42 table models / 42 unique owners; suite `accounting/foundation` owns 10
  tables (legal entities, fiscal periods, posting accounts, immutable posted
  documents/lines, journals/lines, idempotency, audit, document artifacts).
- Only the foundation has a **delivered, ordered migration set** with app-owned
  Accounting/RLS controls, accepted live as non-superuser. Service providers
  (invoice 2, AR 4, AP 4, …) describe registries but have no delivered
  acceptance. See [`README.md`](README.md) "Current persistence status" and
  `docs/roadmap/.../service-readiness-plan/entity-ownership.md`.
- Sesame identity → Lifeguard `SessionContext` → transaction-scoped RLS is
  proven plumbing (invoice, GL); the vendored Sesame RLS contract installs
  before accounting controls with the explicit v1 helper grant set.

### 2.3 Delivery and runtime boundary

- Flux profile: only **GL + Invoice are active Helm releases**; the other 15
  components are suspended `scaffold-only` catalog releases (README "Dev
  deployment boundary"). Activation is explicit by design — keep it that way.
- Build hygiene since #109/#110: all accounting gen+impl crates are in
  `default-members`, `cargo check --workspace --all-targets --locked` 0 errors,
  lib tests + `just qa` green on `main`. The plan's "active-binary gate" gap is
  now half-closed by compilation; the missing half is a **runtime activation
  gate** (see Story A2).

### 2.4 Delta against the roadmap plan's baseline table

| Plan said | Today |
|---|---|
| "full service-workspace check still fails in broad inactive stubs" | Closed — everything compiles (#109) |
| "CI tests workspace libraries only … stale binaries missed" | Closed — `--all-targets` in default-members (#109 + #110) |
| Invoice/GL Partial | Unchanged |
| AR/AP/Bank/Reports "Scaffold only" | **Worse than scaffold-only to outside eyes**: they serve fabricated responses over HTTP (NFR-DOG-013 violation surface) |
| BFF "Partial — publish only delivered operations" | Unchanged; 155 proxy ops advertise stub-and-mock upstreams |

---

## 3. Prerequisites for building out (what must be true before/while coding)

1. **Contract narrowing precedes implementation.** Invoice and GL became real
   only after their OpenAPI was rebuilt into a small honest slice. AR/AP/
   bank-sync/reports specs are still broad research-era contracts (aging with
   DSO/collection-efficiency analytics, 19–25 ops each). Implementing them as
   written means coding fiction. Narrow first, per service, as its own review
   unit ("spec fixes need review" is the #109 lesson).
2. **Decimal money in contracts.** AR/AP/bank specs carry `f64` money fields
   (aging buckets `number`/float). Constraint #10 requires decimal strings +
   explicit currency/rounding, like invoice's canonical contract. This is a
   spec-fix task riding each Epic B item, never a hand-edit in `gen/`.
3. **No mock dispatch in impl binaries.** `register_from_spec` serves generated
   example handlers over HTTP in the four fake services and BFF-adjacent impls.
   NFR-DOG-013 (contract honesty) fails the moment a consumer exists.
4. **GL breadth gates subledgers.** AR/AP correctness depends on GL-owned
   account lifecycle, period open/close/hard-lock, posting/reversal, and
   control-account protection (WP1 dependency graph). Sequence accordingly.
5. **Reuse the acceptance pattern, don't invent a new one.** Invoice's live
   non-superuser PostgreSQL acceptance (post → balanced journal → same-payload
   retry → changed-payload conflict → retrieve → credit note, plus advisory
   lock numbering proof) is the template for every subledger story.
6. **External decisions stay out unless domain-neutral code needs them**
   (accounting-policy ADR: principal-vs-agent/tax point; jurisdiction/tax
   policy; bank sandbox; retention policy). Where absent: configurable,
   domain-neutral contracts + file/mock adapters; never invent jurisdiction or
   bank behavior.
7. **WP0 leftovers are cheap and block expansion**: impl anatomy completion
   (services/validators/seeds/tests per service), platform-vs-accounting SQL
   separation, suite-install isolation tests, migration generation idempotency
   (re-run = no drift).

---

## 4. Story backlog

Sizing is relative (S ≤ 1 day, M ≈ 2–4 days, L ≈ 1–2 weeks). `deps` = blocked
by. IDs are stable; reference them in commits.

### Epic A — Runtime truthfulness (do first, whole-suite, cheap)

- **A1 — Replace mock dispatch with honest 501s** (L) — **DONE 2026-10-28** (PR #111)
  As a future consumer, I want undelivered operations to fail honestly, so
  that no client can ever be fooled by example data.
  Remove `registry::register_from_spec(...)` mock wiring from
  `accounts-receivable|accounts-payable|bank-sync|financial-reports`
  `main.rs`; replace TODO-stub controllers' fabricated bodies with a shared
  `http_support` "not implemented" response (typed, 501, operation name).
  *AC*: `grep -rn register_from_spec microservices/accounting/*/impl/src/main.rs`
  empty; every AR/AP/bank/report route returns 501 with stable error body;
  no controller returns literal example values; tests assert 501 per op group.
- **A2 — Runtime activation gate** (M, deps A1) — **DONE 2026-10-28** (PR #111)
  As a maintainer, I want CI to fail if a scaffold-only service ever serves
  business-looking responses again, so that honesty is mechanical, not tribal.
  Add a suite-local check (tooling test): for services not in the delivered
  set {general-ledger, invoice}, assert controllers contain no fabricated
  response literals and main.rs has no spec-mock dispatch.
  *AC*: check passes; re-introducing a fabricated response in any scaffold
  crate fails it; wired into `just qa`.
- **A3 — BFF delivered-operations filter** (M, deps A1)
  As a BFF consumer, I want only upstreams that actually answer, so that
  aggregate 200s never hide stub 501s. BFF stays a proxy (its 501s from A1
  pass through honestly); document + test the propagation for one op per
  service. *AC*: bff test shows pass-through 501 for a stub op and 200 for a
  delivered GL op.

### Epic B — Contract narrowing (per service, spec-first)

Per service {AR, AP, bank-sync, financial-reports}: narrow `openapi/accounting/<svc>/openapi.yaml`
to the first-vertical operation set; convert money fields to decimal strings
with explicit currency; regenerate `gen/` via `rerp gen suite`; re-sync impl
stubs (`rerp gen stubs --force` after proving every overwritten file is a
pure stub); commit spec vs gen vs impl separately (enforced convention).

- **B1 — AR contract slice** (M, deps A1) — **DONE 2026-10-28** (PR this slice): issue-visible ledger query, receipt
  record, allocation, allocation reversal, aging (decimal buckets), statement
  request. Drops: DSO, collection-efficiency, predictive analytics.
- **B2 — AP contract slice** (M, deps A1) — **DONE 2026-10-28**: bill capture/post, credit note,
  approval transition, payment record, allocation (+reversal), aging.
- **B3 — bank-sync contract slice** (M, deps A1) — **DONE 2026-10-28**: bank-account registry,
  statement ingestion (provider-idempotent), transaction list, match/override,
  reconciliation status. Drops: institution-specific ops, FX hedging.
- **B4 — financial-reports contract slice** (S, deps B1,B2) — **DONE 2026-10-28**: trial balance,
  P&L, balance sheet, AR/AP aging reports — each with explicit tenant/legal
  entity/currency/period semantics in the schema.
- **B5 — Decimal-money sweep** (S, rides B1–B4): `grep` audit of all accounting
  specs for `format: float|double`/`number` on money paths; zero results is the
  gate; add to A2's tooling check.

### Epic C — WP0 structural close-out (parallel, small)

- **C1 — Impl anatomy completion** (M): create services/validators/config/
  seeds/tests scaffolding per the canonical layout in CONTRIBUTING for the four
  scaffold services, so Epic F–I code lands in the right places.
- **C2 — Platform-vs-accounting SQL separation** (M): split shared platform
  prerequisites from Accounting-specific controls in the DB setup path.
- **C3 — Suite-install isolation tests** (M, deps C2): fresh-install tests for
  Accounting-only, Documents-only, combined against native PostgreSQL
  (NFR-DOG-009).
- **C4 — Migration idempotency check** (S): re-run generation with no model
  change → zero diff, in CI.

### Epic D — GL mutation breadth (WP1 remainder)

- **D1 — Chart of accounts lifecycle** (L): account create/type/currency/
  posting-control mutations with tenant-scoped authorization; control-account
  protection for AR/AP/bank/tax/retained-earnings.
- **D2 — Fiscal period state machine** (M): open/close/reopen/hard-lock;
  *AC*: concurrent posting into closed period rejected deterministically
  (live RLS acceptance pattern).
- **D3 — Posting & reversal service** (L, deps D1,D2): generic balanced journal
  post + linked reversal preserving both audit histories; deterministic
  legal-entity/year numbering extended from invoice's advisory-lock scheme.
- **D4 — Opening balances & year transition policy** (M, deps D3).
- **D5 — Trial balance dimensions** (M, deps D3): by period, currency,
  supported dimensions; keeps the existing integrity pre-validation.

### Epic E — Invoice hardening (WP2 remainder)

- **E1 — Durable render outbox** (M): atomically store immutable render model +
  Documents-Render outbox instruction at posting; posting succeeds while
  rendering is down (NFR-DOG-010).
- **E2 — Partial credit/debit notes** (M): explicit workflows only; original
  stays immutable.
- **E3 — Generated rustls client proof** (M): generated HTTPS consumer covers
  post/retrieve/credit incl. retry + payload-conflict paths (goal 6 close).

### Epic F — AR vertical (first real subledger; deps D3, B1, C1)

- **F1 — Customer ledger from posted facts** (L, deps D3): open items sourced
  from invoice postings + credit notes; entity migrations for AR's 4 models
  delivered with live RLS acceptance.
- **F2 — Receipts + allocations** (L, deps F1): authoritative external payment
  reference; full/partial/multi-doc allocation; allocation reversal with audit
  linkage; *AC*: allocations cannot exceed balances; invoice+credit+receipt+
  allocation+outstanding identities hold.
- **F3 — Aging & statements** (M, deps F2): reproducible as-of aging
  (decimal), customer statement via Documents Render, agrees with subledger
  and GL control account.

### Epic G — AP vertical (deps D3, B2, C1)

- **G1 — Bill capture + posting** (L, deps D3): supplier identity, duplicate
  supplier-doc detection in tenant scope, approval state separate from posting
  state (authorization cannot be bypassed by generic status update).
- **G2 — Payments + allocations** (L, deps G1): proposal/batch with explicit
  authorization + audit; reversal preserves history; aging agrees with
  supplier open items and GL control account.

### Epic H — Banking & reconciliation (WP5, mock-first)

- **H1 — Registry + idempotent ingestion** (L, deps F2 or G1): bank/account
  registry with GL mapping; statement/transaction import with stable provider
  identity + payload checksum (re-import cannot duplicate; changed payload
  fails visibly).
- **H2 — File/mock connector** (M, deps H1): dev + contract-acceptance
  connector behind the connector interface (auth, cursor, retry, error
  normalization).
- **H3 — Matching & reconciliation** (L, deps H2): deterministic matching for
  invoices/bills/payments/fees/interest; manual match/override with reasons;
  partial/one-to-many; completion + reopen audit; fees/interest post explicit
  journals, never mutate imports.
- **H4 — First real bank adapter** (—, **externally blocked**): do not start
  until the plan's institution checklist (official API docs, sandbox
  agreement, credentials) is recorded. Keep out of estimates.

### Epic I — Financial reporting (WP6, deps D3, F3, G2, H3)

- **I1 — Core reports** (L): trial balance, P&L, balance sheet, cash position;
  posted/reconciled facts only; P&L+BS reconcile to the same trial balance;
  deterministic re-runs.
- **I2 — Subledger reports** (M, deps I1): AR/AP ledgers + aging through the
  reports service; reconcile to GL control accounts or expose explicit
  exception.

### Epic J — BFF + consumer proof (WP8 close-out, deps Epic A + F/G/H/I as delivered)

- **J1 — Delivered-only BFF surface** (M): suite config publishes only delivered
  ops; generation fails on path/operation/schema clashes (the #533
  prefix/rewrite machinery already works).
- **J2 — Contract fixtures + mock server** (M): public-contract fixtures so
  Hauliage-side work proceeds without touching RERP internals.
- **J3 — Order-to-cash + procure-to-pay dogfood scenarios** (L): the plan's
  WP8 acceptance scenarios executed end-to-end over HTTPS by the generated
  client, with cross-tenant failure paths proven at API *and* DB layer.

### Epic K — External decisions (track, don't code)

- **K1** Accounting-policy ADR (principal-vs-agent, invoice parties, tax point,
  settlement events) — joint Hauliage/RERP.
- **K2** Initial jurisdiction/tax/VAT policy + currencies + rounding source.
- **K3** First-bank product/sandbox selection (feeds H4).
- **K4** Object-storage retention/encryption policy.

### Recommended sequencing

```
A1 ─→ A2,A3 ─→ B1 ─→ D1,D2 ─→ D3 ─→ F1 → F2 → F3 ─→ G1 → G2
        B2 rides D      (C1–C4 in parallel; E1–E3 any time)
D3 + H1–H3 ─→ I1,I2 ─→ J1–J3        H4 awaits K3        K1,K2 gate tax rules
```

The single highest-leverage first move is **A1**: a day of honest 501s turns
the whole suite's status table from *misleading* to *readable*, and every
later story then lands against a truthful surface.

---

## 5. Standing constraints for every story (from the plan; binding)

1. `impl/` only — never hand-edit `gen/`; specs change via spec PRs.
2. One `LifeModel` owner + one migration provider per table; suite foundation
   only for ownerless concepts.
3. Migrator: explicit suite selection, writes only to
   `microservices/<suite>/migrations/`.
4. Sesame-validated identity → Lifeguard transaction-scoped RLS; caller never
   picks tenant/legal entity in a body.
5. No `execute_unprepared` escapes; typed Lifeguard APIs.
6. `may_minihttp::HttpClient` rustls stack in production paths; tests use the
   same transport contract.
7. Decimal money, explicit currency + rounding policy, no binary floats.
8. Posted facts immutable; corrections are explicit document workflows.
9. Conventional commits + pre-commit hooks without bypass; new tables get
   migrations + live non-superuser acceptance in the same PR.
10. Scaffold-only services stay out of active Flux releases until the delivery
    acceptance for them is durable.

---

## Appendix — how this was verified (2026-10-28, branch `main`)

```bash
# per-service reality
for s in general-ledger invoice accounts-receivable accounts-payable \
         bank-sync financial-reports bff; do
  echo "$s: $(find microservices/accounting/$s/impl/src -name '*.rs' | xargs wc -l | tail -1)"
  grep -rn "register_from_spec" microservices/accounting/$s/impl/src/main.rs
done
sed -n '1,30p' microservices/accounting/accounts-receivable/impl/src/controllers/aging_summary.rs
# foundation + delivery boundary: microservices/accounting/README.md
# gap map: docs/roadmap/hauliage-accounting-dogfood/service-readiness-plan/README.md
# build hygiene: cargo check --workspace --all-targets --locked (0 errors)
```
