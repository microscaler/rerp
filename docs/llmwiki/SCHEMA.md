# RERP LLM Wiki Schema

## Purpose

This wiki is a persistent knowledge layer for RERP agents. It stores reconciled, operational knowledge that should survive chat boundaries: code conventions, tooling contracts, known drift, and decisions that are easy for future agents to accidentally undo.

## Source of Truth Order

When claims disagree, use this order:

1. Current code and runtime behavior: `tooling/`, `microservices/`, `openapi/`, `entities/`, `Tiltfile`, CI workflows.
2. Generator inputs and generated artifacts: `openapi/**`, `microservices/**/gen`, BRRTRouter sibling tooling.
3. Human-authored docs: `AGENTS.md`, `docs/`, ADRs, PRDs, design proposals.
4. This wiki as the reconciled synthesis.

If the wiki contradicts code or higher-ranked docs, update the wiki. Do not silently let the wiki override code.

## Layout

```text
docs/llmwiki/
├── SCHEMA.md
├── README.md
├── index.md
├── log.md
├── docs-catalog.md
├── topics/
├── entities/
└── reconciliation/
```

Keep `topics/`, `entities/`, and `reconciliation/` flat. Prefer updating an existing page over creating another near-duplicate.

## Page Conventions

Substantive pages start with:

```text
# <Title>

- **Status**: `verified` | `partially-verified` | `unverified`
- **Source docs**: repo-relative paths
- **Code anchors**: repo-relative paths
- **Last updated**: YYYY-MM-DD
```

Then include:

- **What it is**: short explanation.
- **Where it lives / how it works**: file anchors and key commands.
- **Gotchas / drift**: explicit failure modes and migration state.
- **Cross-references**: related wiki/source pages.

## Operations

### Ingest

When work changes code or authoritative docs:

1. Identify which wiki pages are affected.
2. Update or create only the relevant pages.
3. Update `index.md` for new pages.
4. Append a `log.md` entry: `## [YYYY-MM-DD] ingest | <short>`.
5. Mark contradictions with `> **Drift:**` or `> **Open:**` instead of burying them.

### Query

When answering architecture/tooling questions:

1. Read `index.md`.
2. Read relevant topic/entity pages.
3. Verify uncertain claims against code.
4. File durable findings back into the wiki.

### Session Close

At the end of meaningful work, update touched pages and append to `log.md`. This is part of keeping RERP recoverable across agent crashes and context compaction.
