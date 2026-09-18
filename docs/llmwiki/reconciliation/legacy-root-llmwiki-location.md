# Legacy Root LLM Wiki Location

- **Status**: `partially-verified`
- **Source docs**: [`AGENTS.md`](../../../AGENTS.md), `llmwiki/`
- **Code anchors**: `docs/llmwiki/`, `llmwiki/`
- **Last updated**: 2026-04-25

## What It Is

This checkout contains an older root-level `llmwiki/` tree as well as the canonical `docs/llmwiki/` tree added per `AGENTS.md` documentation rules.

The root-level tree has useful material, but `AGENTS.md` says planning, analysis, design, and long-lived agent knowledge must live under `docs/` or its subdirectories. Future wiki work should therefore target `docs/llmwiki/`.

## Current Reconciliation Rule

- Use `docs/llmwiki/` as the canonical wiki location.
- Treat root `llmwiki/` as historical source material until it is migrated or removed by an explicit cleanup task.
- Do not add new pages to root `llmwiki/`.
- When root `llmwiki/` has a useful page, ingest/summarize it into `docs/llmwiki/` and cite the current code/docs anchors.

## Known Imported Themes

The first `docs/llmwiki/` seed imported or superseded these root-level themes:

- Runtime/codegen/service flow → [`topics/hauliage-reference-operating-model.md`](../topics/hauliage-reference-operating-model.md)
- Lifeguard/entity/database notes → [`topics/service-implementation-and-database-layout.md`](../topics/service-implementation-and-database-layout.md)
- RERP CLI tooling guardrails → [`topics/suite-aware-brrtrouter-wrapper.md`](../topics/suite-aware-brrtrouter-wrapper.md)

## Open

> **Open:** Decide whether to migrate all remaining root `llmwiki/` pages into `docs/llmwiki/` and then remove the root tree. Until then, this reconciliation page exists to prevent divergent updates.
