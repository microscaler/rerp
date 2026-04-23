# RERP LLM Wiki Schema

## Purpose
This wiki is a persistent, code-anchored knowledge layer between `docs/` and the
Rust microservice codebase. It compiles cross-service knowledge that no single
OpenAPI spec or service crate can express alone.

## Source of Truth Order
1. Runtime behavior in `microservices/{suite}/{service}/gen/` and `impl/`
2. Generated behavior from `openapi/{suite}/{name}/openapi.yaml` via `brrtrouter-gen`
3. Existing prose docs in `docs/` (ADRs, PRDs, AI analysis)
4. This wiki (`llmwiki/`) as reconciled synthesis

## Page Conventions
- Every substantive page includes:
  - **Status** (`verified`, `partially-verified`, `unverified`)
  - **Source docs** (`docs/...` links)
  - **Code anchors** (absolute repository paths)
  - **Gaps / drift** (doc claim vs code reality)
- Prefer explicit file paths, OpenAPI paths, and function names over high-level claims.
- Keep operational instructions executable and minimal.
- Cross-link sibling pages with `./page-name.md` links.

## Operational Workflows
- **Ingest**: add/refresh entries from `docs/**` and `openapi/**` into
  `llmwiki/docs-catalog.md`, then reconcile with code.
- **Query**: answer from `llmwiki/index.md` + linked pages first, then verify
  in code when uncertain.
- **Lint**: regularly check for stale claims and unresolved gaps in
  `llmwiki/reconciliation/*.md`.

## Logging
- Append session updates to `llmwiki/log.md`.
- Keep entries chronological and append-only.
- Entry format: `## [YYYY-MM-DD] category | summary`
