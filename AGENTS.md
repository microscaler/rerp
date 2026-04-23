# RERP — agent rules

> **Desktop dev environment** — before doing anything in this repo, read the
> Microscaler-wide topology brief. It explains that you are on a Mac but the
> code lives on `ms02` (NFS), where commands execute for this environment, how
> the Kind cluster and vLLM fit in, and the network constraints behind the SSH
> tunneling. Do not duplicate its contents here — link to it. If reality drifts,
> fix the canonical doc, not this copy.
>
> - GitHub: [`cylon-local-infra/docs/desktop-dev-environment.md`](https://github.com/microscaler/cylon-local-infra/blob/main/docs/desktop-dev-environment.md)
> - On ms02 NFS: `~/Workspace/microscaler/cylon-local-infra/docs/desktop-dev-environment.md`

---

Strict operational rules for AI assistants working in this repository. **Knowledge about how RERP works is in [`llmwiki/`](./llmwiki/), not here.** This file only holds rules the agent must obey.

---

## Before you do anything

1. Read [`llmwiki/index.md`](./llmwiki/index.md) — the content catalog.
2. Read [`llmwiki/SCHEMA.md`](./llmwiki/SCHEMA.md) — wiki conventions and workflows.
3. Tail [`llmwiki/log.md`](./llmwiki/log.md) for recent context from other sessions.
4. Read the specific topic / entity / reference pages relevant to your work. Drill into prose `docs/` or source only when the wiki flags drift or a gap.

Sessions that skip this waste work. The wiki is the compounding artifact that makes knowledge persist across sessions — see [Karpathy's llm-wiki gist](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f).

---

## Repository shape

- Primary language: Rust (workspace with 71 services × 2 crates = 142 crates).
- Each service: `openapi/{suite}/{name}/openapi.yaml` (spec) → `microservices/{suite}/{name}/gen/` (generated) → `microservices/{suite}/{name}/impl/` (business logic).
- 71 OpenAPI specs, 71 generated crates, 71 implementation crates.
- Frontend: `frontend/` (Vite), bypasses BFF via proxy.
- UI: `ui/shared/`, `ui/website/`.
- Sibling repos (typical `microscaler/` checkout): **`../brrtrouter/`** (HTTP router + codegen) — [`llmwiki/`](../brrtrouter/llmwiki/). **`../lifeguard/`** (ORM + migrations) — [`docs/llmwiki/`](../lifeguard/docs/llmwiki/). **`../hauliage/`** (primary HTTP consumer + legacy BFF) — [`docs/llmwiki/`](../hauliage/docs/llmwiki/).

---

## Build, lint, test commands

### Setup

- `just init` — Set up `tooling/.venv` with `rerp` CLI and deps.

### Recommended entry points (justfile)

- `just build` — Build workspace.
- `just test` — Run tests.
- `tooling/.venv/bin/rerp` — Main automation CLI.

### Build

- `cargo build` — Build default workspace.
- `cargo build --workspace` — Explicit workspace build.
- `cargo build -p <crate-name>` — Build a specific crate.

### Format / lint

- `cargo fmt` — Format Rust (always run before committing).
- `cargo clippy --workspace --all-targets --all-features` — Lint.

### Test

- `cargo test --workspace` — Run all workspace tests.
- `cargo test -p <crate-name>` — Single crate tests.

### Code generation

- `brrtrouter-gen --spec openapi/{suite}/{name}/openapi.yaml --output microservices/{suite}/{name}/gen` — Regenerate a single service.
- `rerp bff generate-system` — Generate system-level BFF specs (via `tooling/.venv/bin/rerp`).

### Suite BFF generation

- `bff-generator generate-spec --config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml`

---

## Core rules the agent must obey

### 1. Two-crate model — do not edit generated code

Each service has `gen/` (auto-generated from OpenAPI) and `impl/` (business logic). **Never edit `gen/` directly.** Fix the OpenAPI spec or templates, then regenerate with `brrtrouter-gen`.

### 2. OpenAPI-first

All API changes must start in `openapi/{suite}/{name}/openapi.yaml`. Changes to the API surface require regenerating the gen crate. Business logic changes go directly in `impl/src/`.

### 3. Lifeguard entity migration — NEVER edit migrations directly

Updating entity structs in `entities/src/` is the only valid way to change the database schema. The migrator compares entity definitions against existing migrations and produces additive deltas. Directly editing migration files will produce broken migrations.

Index rules: `#[index = "idx_name(column)"]` must reference only columns that exist on the struct. Child entities do NOT inherit base entity columns. To query by base-entity fields from a child table, use a JOIN.

### 4. Follow Rust conventions

- `snake_case` for fns / modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Group imports: std, external crates, local modules. Prefer explicit imports.
- `Result<T, E>` + `?` over `panic!` in library paths.

### 5. Documentation expectations

- Public modules require `//!` module-level docs.
- Public fns / structs / enums / traits require `///` docs.
- Keep prose docs in `docs/` and keep the wiki in `llmwiki/` up to date.

---

## Commit discipline

- Commits follow Conventional Commits (`feat(scope):`, `fix(scope):`, `docs(scope):`, `chore(scope):`, `refactor(scope):`).
- **Never push** without explicit human authorization.
- **Never use `--no-verify`** or `--no-verify-commit`. Let pre-commit hooks run.
- **Never commit secrets** (`.env`, credentials, tokens).
- **Check staged files** before committing — must not be in `.gitignore` or be generated/temporary artifacts.

---

## Suite conventions

Suites are inferred **dynamically** from `openapi/{suite}/bff-suite-config.yaml`. No hardcoded suite names exist in ports or tooling. Adding a new suite only requires adding its `bff-suite-config.yaml`.

### Current suite: accounting

The only suite with a generated BFF:
- Config: `openapi/accounting/bff-suite-config.yaml`
- Output: `openapi/accounting/openapi_bff.yaml`
- 9 microservices: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports

---

## Known issues (reference)

- **Draft consignments 403**: `POST /api/v1/consignments/jobs/drafts` returns 403 — draft endpoints have no security schemes in OpenAPI but brrtrouter default auth may block them.
- **Company addresses 502**: `GET /api/v1/company/organizations/me/addresses` returns 502 — stub impl returns `[]`, no `AddressBookEntry` model exists.
- **BFF non-functional**: Frontend bypasses BFF entirely via vite proxy. BFF redesign is a future priority.

---

## Useful files

- [`README.md`](./README.md) — project overview.
- [`CONTRIBUTING.md`](./CONTRIBUTING.md) — contributor workflow.
- [`RERP_MUSINGS.md`](./RERP_MUSINGS.md) — module breakdown and market analysis.
- [`microservices/Cargo.toml`](./microservices/Cargo.toml) — workspace root config.
- [`port-registry.json`](./port-registry.json) — port registry for dev services.
- [`tooling/README.md`](./tooling/README.md) — `rerp` CLI reference.
- [`docs/EXECUTIVE_SUMMARY.md`](./docs/EXECUTIVE_SUMMARY.md) — executive overview.
- [`docs/ai/OPENAPI_GENERATION_COMPLETE.md`](./docs/ai/OPENAPI_GENERATION_COMPLETE.md) — 71 services generation status.
- [`docs/ai/BFF_GENERATION_COMPLETE.md`](./docs/ai/BFF_GENERATION_COMPLETE.md) — BFF generation status.
- [`docs/adrs/`](./docs/adrs/) — Architecture Decision Records.
- [`docs/ai/MICROSERVICE_MATRIX_AUDIT.md`](./docs/ai/MICROSERVICE_MATRIX_AUDIT.md) — 71-service matrix audit.
- [`llmwiki/`](./llmwiki/) — wiki catalog and compounding knowledge layer.

---

## Explicit instruction: read the wiki

**Every session starts with reading [`llmwiki/`](./llmwiki/).** This is not optional.

End-of-session: update the wiki pages your work touched, append a `log.md` entry, flag any `> **Open:**` questions. Leave the wiki one step more useful than you found it. The convention for RERP features is: every PRD / feature commit adds or extends 1–3 wiki pages tied to the work, cross-linked to sibling repos where concepts span both RERP and hauliage / lifeguard / brrtrouter.
